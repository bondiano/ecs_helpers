use std::time::Duration;

use aws_sdk_ecs::types::Service;
use ecs_helpers::{
  args::DeployCommandArguments, cluster_helpers, config::Config, ecr::EcrClient, ecs::EcsClient,
  errors::EcsHelperVarietyError, service_helpers, Command,
};

const DEFAULT_STEP: u64 = 5;

pub struct DeployCommand {
  ecs_client: EcsClient,
  ecr_client: EcrClient,
  config: Config,
  timeout: u64,
  cluster: Option<String>,
  service: Option<String>,
}

impl DeployCommand {
  pub fn new(config: Config, args: DeployCommandArguments) -> Self {
    let sdk_config = &config.sdk_config;
    let ecs_client = EcsClient::new(sdk_config);
    let ecr_client = EcrClient::new(sdk_config);

    Self {
      ecs_client,
      ecr_client,
      config,
      timeout: args.timeout,
      cluster: args.cluster,
      service: args.service,
    }
  }

  async fn wait_for_deploy(
    &self,
    cluster_arn: &String,
    service_arn: &String,
  ) -> miette::Result<Service, EcsHelperVarietyError> {
    let mut timeout = self.timeout;

    while timeout > 0 {
      let service = self
        .ecs_client
        .describe_service(cluster_arn, service_arn)
        .await?;

      let deployment_count = service.deployments().len();

      if deployment_count == 1 {
        log::info!("Service was deployed");
        return Ok(service);
      }

      timeout -= DEFAULT_STEP;
      tokio::time::sleep(Duration::from_secs(DEFAULT_STEP)).await;
    }

    Err(EcsHelperVarietyError::WaitTaskTimeoutError(self.timeout))
  }
}

impl Command for DeployCommand {
  fn name(&self) -> String {
    "deploy".to_string()
  }

  async fn run(&self) -> Result<(), EcsHelperVarietyError> {
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
            .ecr_client
            .create_new_container_definition_from(
              container_definition,
              repositories,
              &self.config.version,
            )
            .await
        }
      }))
      .await;
    let new_container_definitions = container_definitions_to_ecr_tasks
      .iter()
      .filter_map(
        |container_definition_to_ecr| match container_definition_to_ecr {
          Ok(container_definition_to_ecr) => Some(container_definition_to_ecr.to_owned()),
          Err(_) => None,
        },
      )
      .collect::<Vec<_>>();

    let new_service_task_definition = self
      .ecs_client
      .register_task_definition_from(&service_task_definition, new_container_definitions)
      .await?;

    log::info!("Register task definition\nTask definition was registered",);

    let service_task_definition_arn = new_service_task_definition
      .task_definition_arn()
      .unwrap()
      .to_owned();
    let service_arn = service.service_arn().unwrap().to_owned();
    let cluster_arn = service.cluster_arn().unwrap().to_owned();

    let service = self
      .ecs_client
      .update_service(&cluster_arn, &service_task_definition_arn, &service_arn)
      .await?;

    log::info!("Update service\nService task definition was updated");

    let service_arn = service.service_arn().unwrap().to_owned();
    let cluster_arn = service.cluster_arn().unwrap().to_owned();

    self.wait_for_deploy(&cluster_arn, &service_arn).await?;

    log::info!("Success\nApplication was successfully deployed");

    Ok(())
  }
}
