use std::time::Duration;

use aws_sdk_ecr::types::{ImageIdentifier, Repository};
use aws_sdk_ecs::types::{ContainerDefinition, Task};
use ecs_helpers::{
  args::RunCommandArguments, cluster_helpers, config::Config, ecr::EcrClient, ecs::EcsClient,
  errors::EcsHelperVarietyError, service_helpers, Command,
};

const DEFAULT_STEP: u64 = 5;
const STOPPED_STATUS: &str = "STOPPED";

#[derive(Debug, Clone)]
pub struct RunCommandCommand {
  ecs_client: EcsClient,
  ecr_client: EcrClient,
  config: Config,
  command: String,
  timeout: u64,
  cluster: Option<String>,
  service: Option<String>,
  name: Option<String>,
  container: Option<String>,
}

impl Command for RunCommandCommand {
  fn name(&self) -> String {
    "run_command".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let cluster =
      cluster_helpers::get_current_cluster(&self.ecs_client, &self.config, &self.cluster).await?;

    let service =
      service_helpers::get_current_service(&self.ecs_client, &self.config, &cluster, &self.service)
        .await?;

    let service = self.ecs_client.describe_service(&cluster, &service).await?;

    let service_task_definition = service.task_definition().unwrap().to_owned();

    let service_task_definition = self
      .ecs_client
      .describe_task_definition(&service_task_definition)
      .await?;

    let repositories = self.ecr_client.get_private_repositories().await?;

    let container_definitions = service_task_definition.container_definitions().to_vec();

    let container_definitions_to_ecr_tasks =
      futures::future::join_all(container_definitions.iter().map(|container_definition| {
        let repositories = repositories.clone();

        async move {
          self
            .container_definition_to_ecr(repositories, container_definition)
            .await
        }
      }))
      .await;

    let container_definitions_to_ecr = container_definitions_to_ecr_tasks
      .iter()
      .filter_map(
        |container_definition_to_ecr| match container_definition_to_ecr {
          Ok(container_definition_to_ecr) => Some(container_definition_to_ecr.to_owned()),
          Err(_) => None,
        },
      )
      .collect::<Vec<_>>();

    let mut new_container_definition = container_definitions_to_ecr
      .iter()
      .find(|container_definition| {
        let container_name = match container_definition.name() {
          Some(container_name) => container_name,
          None => return false,
        };

        match &self.container {
          Some(container) => container_name.contains(container),
          None => true,
        }
      })
      .unwrap_or(container_definitions_to_ecr.first().unwrap())
      .to_owned();

    let container_name = new_container_definition.name().unwrap();
    let name = match self.name {
      Some(ref name) => format!("{container_name}-{name}"),
      None => container_name.to_string(),
    };

    let new_log_configuration = new_container_definition
      .log_configuration()
      .unwrap()
      .to_owned();
    let mut new_options = new_log_configuration.options().unwrap().to_owned();
    let new_log_configuration_prefix = format!(
      "{}-{name}",
      new_log_configuration
        .options()
        .unwrap()
        .get("awslogs-stream-prefix")
        .unwrap()
    );
    new_options.insert(
      "awslogs-stream-prefix".to_string(),
      new_log_configuration_prefix,
    );

    new_container_definition.log_configuration = Some(new_log_configuration);
    new_container_definition.name = Some(name);
    new_container_definition.command = Some(vec![
      "bash".to_string(),
      "-c".to_string(),
      self.command.clone(),
    ]);

    let new_service_task_definition = self
      .ecs_client
      .register_task_definition_from(&service_task_definition, &new_container_definition)
      .await?;

    let task_definition_arn = new_service_task_definition
      .task_definition_arn()
      .unwrap()
      .to_string();
    let network_configuration = service.network_configuration().unwrap().to_owned();

    let task = self
      .ecs_client
      .run_task(
        &cluster,
        &task_definition_arn,
        &network_configuration,
        service.launch_type(),
      )
      .await?;

    log::info!("Start task: {}", &task.task_arn().unwrap());

    let task_arn = task.task_arn().unwrap().to_owned();

    self.wait_for_task(&task_arn, &cluster).await?;

    Ok(())
  }
}

impl RunCommandCommand {
  pub fn new(config: Config, options: RunCommandArguments) -> Self {
    let sdk_config = &config.sdk_config;
    let ecs_client = EcsClient::new(sdk_config);
    let ecr_client = EcrClient::new(sdk_config);

    Self {
      ecs_client,
      ecr_client,
      config,
      name: options.name,
      timeout: options.timeout,
      cluster: options.cluster,
      service: options.service,
      command: options.command,
      container: options.container,
    }
  }

  async fn container_definition_to_ecr(
    &self,
    repositories: Vec<Repository>,
    container_definition: &ContainerDefinition,
  ) -> miette::Result<ContainerDefinition, EcsHelperVarietyError> {
    let repository = repositories
      .iter()
      .find(|repository| {
        let repository_uri = match repository.repository_uri() {
          Some(repository_uri) => repository_uri,
          None => return false,
        };

        let image = match container_definition.image() {
          Some(image) => image,
          None => return false,
        };

        image.contains(repository_uri)
      })
      .unwrap();

    let repository_name = repository.repository_name().unwrap();
    let repository_uri = repository.repository_uri().unwrap();
    let version = self.config.version.clone();

    let ecr_base = repository_uri.split('/').collect::<Vec<_>>();
    let ecr_base = ecr_base.first().unwrap();

    container_definition.image().unwrap().contains(ecr_base);
    let image_identifier = ImageIdentifier::builder().image_tag(&version).build();
    self
      .ecr_client
      .describe_images(repository_name, image_identifier)
      .await?;

    let mut new_container_definition = container_definition.clone();

    // we're partially cloning container definition because we need to change image according to repository
    new_container_definition.image = Some(format!("{repository_uri}:{version}"));

    Ok(new_container_definition)
  }

  async fn wait_for_task(
    &self,
    task_arn: &String,
    cluster_arn: &String,
  ) -> miette::Result<Task, EcsHelperVarietyError> {
    let mut timeout = self.timeout;

    while timeout > 0 {
      let task = self.ecs_client.describe_task(task_arn, cluster_arn).await?;

      let last_status = task.last_status().unwrap();

      if last_status == STOPPED_STATUS {
        let container = task.containers().first().unwrap();

        match container.exit_code().unwrap() {
          0 => {
            log::info!("Task was successful");
            return Ok(task);
          }
          _ => {
            log::error!("Task was failed");
            return Err(EcsHelperVarietyError::TaskWasFailed {
              task_arn: task_arn.to_owned(),
              code: container.exit_code().unwrap(),
            });
          }
        }
      }

      timeout -= DEFAULT_STEP;
      tokio::time::sleep(Duration::from_secs(DEFAULT_STEP)).await;
    }

    Err(EcsHelperVarietyError::WaitTaskTimeoutError(self.timeout))
  }
}
