use aws_config::SdkConfig;
use aws_sdk_ecs::{
  types::{ContainerDefinition, LaunchType, NetworkConfiguration, Service, Task, TaskDefinition},
  Client,
};

use crate::errors::EcsHelperVarietyError;

#[derive(Debug, Clone)]
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
    container_definitions: &ContainerDefinition,
  ) -> miette::Result<TaskDefinition, EcsHelperVarietyError> {
    let mut request = self
      .client
      .register_task_definition()
      .container_definitions(container_definitions.clone());

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

    if let Some(compatibility) = task_definition.compatibilities().first() {
      request = request.requires_compatibilities(compatibility.to_owned());
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

  pub async fn run_task(
    &self,
    cluster_arn: &String,
    task_definition_arn: &String,
    network_configuration: &NetworkConfiguration,
    launch_type: Option<&LaunchType>,
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
}
