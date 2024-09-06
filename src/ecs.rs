use aws_config::SdkConfig;
use aws_sdk_ecs::{
  types::{ContainerDefinition, DesiredStatus, LaunchType, NetworkConfiguration, Service, Session, Task, TaskDefinition},
  Client,
};

use crate::errors::EcsHelperVarietyError;

pub struct EcsClient {
  client: Client,
}

impl EcsClient {
  pub fn new(config: &SdkConfig) -> Self {
    Self {
      client: Client::new(config),
    }
  }

  pub async fn get_clusters(&self) -> miette::Result<Vec<String>, EcsHelperVarietyError> {
    let response = self
      .client
      .list_clusters()
      .max_results(100)
      .send()
      .await
      .map_err(EcsHelperVarietyError::GetListClustersError)?;

    Ok(response.cluster_arns().to_vec())
  }

  pub async fn get_services(
    &self,
    cluster_arn: &String,
  ) -> miette::Result<Vec<String>, EcsHelperVarietyError> {
    let response = self
      .client
      .list_services()
      .cluster(cluster_arn)
      .max_results(100)
      .send()
      .await
      .map_err(EcsHelperVarietyError::GetListServicesError)?;

    Ok(response.service_arns().to_vec())
  }

  pub async fn get_tasks(
    &self,
    cluster_arn: &String,
    service_arn: &String,
  ) -> miette::Result<Vec<String>, EcsHelperVarietyError> {
    let response = self
      .client
      .list_tasks()
      .cluster(cluster_arn)
      .service_name(service_arn)
      .desired_status(DesiredStatus::Running)
      .max_results(100)
      .send()
      .await
      .map_err(EcsHelperVarietyError::GetListTasksError)?;

    Ok(response.task_arns().to_vec())
  }

  pub async fn describe_service(
    &self,
    cluster_arn: &String,
    service_arn: &String,
  ) -> miette::Result<Service, EcsHelperVarietyError> {
    let response = self
      .client
      .describe_services()
      .cluster(cluster_arn)
      .services(service_arn)
      .send()
      .await
      .map_err(EcsHelperVarietyError::DescribeServiceError)?;

    let [service] = response.services() else {
      return Err(EcsHelperVarietyError::NoServicesFound);
    };

    Ok(service.to_owned())
  }

  pub async fn get_task_definitions(&self) -> miette::Result<Vec<String>, EcsHelperVarietyError> {
    let response = self
      .client
      .list_task_definitions()
      .max_results(100)
      .send()
      .await
      .map_err(EcsHelperVarietyError::GetListTaskDefinitionsError)?;

    Ok(response.task_definition_arns().to_vec())
  }

  pub async fn describe_task(
    &self,
    task_arn: &String,
    cluster_arn: &String,
  ) -> miette::Result<Task, EcsHelperVarietyError> {
    let response = self
      .client
      .describe_tasks()
      .cluster(cluster_arn)
      .tasks(task_arn)
      .send()
      .await
      .map_err(EcsHelperVarietyError::DescribeTaskError)?;

    let [task] = response.tasks() else {
      return Err(EcsHelperVarietyError::NoTasksFound);
    };

    Ok(task.to_owned())
  }

  pub async fn describe_task_definition(
    &self,
    task_definition_arn: &String,
  ) -> miette::Result<TaskDefinition, EcsHelperVarietyError> {
    let response = self
      .client
      .describe_task_definition()
      .task_definition(task_definition_arn)
      .send()
      .await
      .map_err(EcsHelperVarietyError::DescribeTaskDefinitionError)?;

    let task_definition = response
      .task_definition()
      .ok_or(EcsHelperVarietyError::ExtractTaskDefinitionError)?;

    Ok(task_definition.to_owned())
  }

  pub async fn register_task_definition_from(
    &self,
    task_definition: &TaskDefinition,
    container_definitions: Vec<ContainerDefinition>,
  ) -> miette::Result<TaskDefinition, EcsHelperVarietyError> {
    let mut request = self
      .client
      .register_task_definition()
      .set_container_definitions(Some(container_definitions));

    if let Some(execution_role_arn) = task_definition.execution_role_arn() {
      request = request.execution_role_arn(execution_role_arn.to_owned());
    }

    if let Some(family) = task_definition.family() {
      request = request.family(family.to_owned());
    }

    if let Some(memory) = task_definition.memory() {
      request = request.memory(memory.to_owned());
    }

    if let Some(network_mode) = task_definition.network_mode() {
      request = request.network_mode(network_mode.to_owned());
    }

    if let Some(cpu) = task_definition.cpu() {
      request = request.cpu(cpu.to_owned());
    }

    if let Some(requires_compatibilities) = task_definition.requires_compatibilities().first() {
      request = request.requires_compatibilities(requires_compatibilities.to_owned());
    }

    if let Some(role_arn) = task_definition.task_role_arn() {
      request = request.task_role_arn(role_arn.to_owned());
    }

    let response = request
      .send()
      .await
      .map_err(EcsHelperVarietyError::RegisterTaskDefinitionError)?;

    let task_definition = response
      .task_definition()
      .ok_or(EcsHelperVarietyError::ExtractTaskDefinitionError)?;

    Ok(task_definition.to_owned())
  }

  pub async fn execute_command(
    &self,
    cluster_arn: &String,
    task_arn: &String,
    container: &String,
    command: &String,
  ) -> miette::Result<Session, EcsHelperVarietyError> {
    let execute_command_builder = self
      .client
      .execute_command()
      .interactive(true)
      .command(command)
      .cluster(cluster_arn)
      .task(task_arn)
      .container(container);

    let response = execute_command_builder
      .send()
      .await
      .map_err(EcsHelperVarietyError::ExecuteCommandError)?;

    let session = response
      .session
      .ok_or(EcsHelperVarietyError::ExtractServiceError)?;

    Ok(session.to_owned())
  }

  pub async fn run_task(
    &self,
    cluster_arn: &String,
    task_definition_arn: &String,
    network_configuration: &NetworkConfiguration,
    launch_type: Option<&'_ LaunchType>,
  ) -> miette::Result<Task, EcsHelperVarietyError> {
    let mut run_task_builder = self
      .client
      .run_task()
      .cluster(cluster_arn)
      .task_definition(task_definition_arn)
      .network_configuration(network_configuration.to_owned());

    if let Some(launch_type) = launch_type {
      run_task_builder = run_task_builder.launch_type(launch_type.to_owned());
    }

    let response = run_task_builder
      .send()
      .await
      .map_err(EcsHelperVarietyError::RunTaskError)?;

    let [task] = response.tasks() else {
      return Err(EcsHelperVarietyError::NoTasksFound);
    };

    Ok(task.to_owned())
  }

  pub async fn update_service(
    &self,
    cluster_arn: &String,
    task_definition_arn: &String,
    service_arn: &String,
  ) -> miette::Result<Service, EcsHelperVarietyError> {
    let response = self
      .client
      .update_service()
      .task_definition(task_definition_arn)
      .service(service_arn)
      .cluster(cluster_arn)
      .send()
      .await
      .map_err(EcsHelperVarietyError::UpdateServiceError)?;

    let service = response
      .service()
      .ok_or(EcsHelperVarietyError::ExtractServiceError)?;

    Ok(service.to_owned())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use aws_config::{BehaviorVersion, Region};
  use aws_sdk_ecr::config::{Credentials, SharedCredentialsProvider};
  use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
  use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
  use aws_smithy_types::body::SdkBody;

  #[tokio::test]
  async fn test_get_clusters() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
        {
          \"clusterArns\": [
            \"arn:aws:ecs:us-east-1:123456789012:cluster/default\"
          ]
        }
      ",
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();
    let ecs_client = EcsClient::new(&sdk_config);

    let clusters = ecs_client.get_clusters().await.unwrap();

    assert_eq!(clusters.len(), 1);
    assert_eq!(
      clusters.first().unwrap(),
      "arn:aws:ecs:us-east-1:123456789012:cluster/default"
    );
  }

  #[tokio::test]
  async fn get_services() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
        {
        \"services\":[]
        }
      ",
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();
    let ecs_client = EcsClient::new(&sdk_config);

    let cluster_arn = "arn:aws:ecs:us-east-1:123456789012:cluster/default".to_owned();

    let clusters = ecs_client.get_services(&cluster_arn).await.unwrap();

    assert_eq!(clusters.len(), 0);
  }

  #[tokio::test]
  async fn get_tasks() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
        {
          \"taskArns\": [
            \"arn:aws:ecs:us-east-1:123456789012:task/cluster/j2h3g4jh32jh23j4hg2j3h4hj2g3j4g\"
          ]
        }
      ",
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();
    let ecs_client = EcsClient::new(&sdk_config);

    let cluster_arn = "arn:aws:ecs:us-east-1:123456789012:cluster/default".to_owned();
    let service_arn = "arn:aws:ecs:us-east-1:123456789012:service/default/web".to_owned();

    let tasks = ecs_client.get_tasks(&cluster_arn, &service_arn).await.unwrap();

    assert_eq!(tasks.len(), 1);
  }

  #[tokio::test]
  async fn test_get_task_definitions() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
        {
          \"taskDefinitionArns\": [
            \"arn:aws:ecs:us-east-1:123456789012:task-definition/nginx:1\"
          ]
        }
      ",
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();
    let ecs_client = EcsClient::new(&sdk_config);

    let task_definitions = ecs_client.get_task_definitions().await.unwrap();

    assert_eq!(task_definitions.len(), 1);
    assert_eq!(
      task_definitions.first().unwrap(),
      "arn:aws:ecs:us-east-1:123456789012:task-definition/nginx:1"
    );
  }

  #[tokio::test]
  async fn test_describe_task() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        r#"
          {
            "tasks": [
              {
                "containers": [
                  {
                    "containerArn": "arn:aws:ecs:us-east-1:123456789012:container/cluster/53df858569f6d3a6e7c0f59b1/46b999999997d-5a58-4de8-a90c-ca9bd338e3a3",
                    "taskArn": "arn:aws:ecs:us-east-1:123456789012:task/cluster/j2h3g4jh32jh23j4hg2j3h4hj2g3j4g",
                    "name": "api",
                    "lastStatus": "RUNNING"
                  }
                ],
                "lastStatus": "RUNNING",
                "taskArn": "arn:aws:ecs:us-east-1:123456789012:task/cluster/j2h3g4jh32jh23j4hg2j3h4hj2g3j4g"
              }
            ]
          }
        "#,
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();

    let ecs_client = EcsClient::new(&sdk_config);
    let cluster_arn = "arn:aws:ecs:us-east-1:123456789012:cluster/default".to_owned();
    let task_arn = "arn:aws:ecs:us-east-1:123456789012:task/cluster/j2h3g4jh32jh23j4hg2j3h4hj2g3j4g".to_owned();
    let container_arn = "arn:aws:ecs:us-east-1:123456789012:container/cluster/53df858569f6d3a6e7c0f59b1/46b999999997d-5a58-4de8-a90c-ca9bd338e3a3".to_owned();

    let task = ecs_client
      .describe_task(&task_arn, &cluster_arn)
      .await
      .unwrap();

    assert_eq!(
      task.containers.unwrap().last().unwrap().container_arn,
      Some(container_arn)
    );
  }

  #[tokio::test]
  async fn test_describe_task_definition() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
        {
          \"taskDefinition\": {
            \"taskDefinitionArn\": \"arn:aws:ecs:us-east-1:123456789012:task-definition/nginx:1\"
          }
        }
      ",
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();

    let ecs_client = EcsClient::new(&sdk_config);
    let task_definition_arn =
      "arn:aws:ecs:us-east-1:123456789012:task-definition/nginx:1".to_owned();
    let task_definition = ecs_client
      .describe_task_definition(&task_definition_arn)
      .await
      .unwrap();

    assert_eq!(
      task_definition.task_definition_arn(),
      Some(task_definition_arn.as_str())
    );
    assert!(task_definition.task_role_arn().is_none());
  }

  #[tokio::test]
  async fn test_update_service() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
        {
          \"service\": {
            \"serviceArn\": \"arn:aws:ecs:us-east-1:123456789012:service/nginx\"
          }
        }
      ",
      ))
      .unwrap();
    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);

    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());

    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();

    let ecs_client = EcsClient::new(&sdk_config);
    let cluster_arn = "arn:aws:ecs:us-east-1:123456789012:cluster/default".to_owned();
    let task_definition_arn =
      "arn:aws:ecs:us-east-1:123456789012:task-definition/nginx:1".to_owned();
    let service_arn = "arn:aws:ecs:us-east-1:123456789012:service/nginx".to_owned();
    let service = ecs_client
      .update_service(&cluster_arn, &task_definition_arn, &service_arn)
      .await
      .unwrap();

    assert_eq!(service.service_arn(), Some(service_arn.as_str()));
  }
}
