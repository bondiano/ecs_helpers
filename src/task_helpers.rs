use crate::{config::Config, ecs::EcsClient, errors::EcsHelperVarietyError};

pub async fn get_current_task(
  ecs_client: &EcsClient,
  config: &Config,
  cluster: &String,
  service: &String,
  task_from_options: &Option<String>,
) -> miette::Result<String, EcsHelperVarietyError> {
  let Config {
    project,
    application,
    environment,
    ..
  } = config;

  let tasks = ecs_client.get_tasks(cluster, service).await?;
  let task_names = tasks.join(", ");

  let task = match task_from_options {
    Some(task_name) =>
      tasks
        .iter()
        .find(|task| task.contains(task_name))
        .ok_or(EcsHelperVarietyError::NoSpecifiedTask(format!(
          "Task specified in CLI does not exist, tasks you have: {task_names}.\nProject: {project}, application: {application}, environment: {environment}, cluster: {cluster}, service: {service}"
        )))?,
    None =>
      tasks
        .iter()
        .find(|task| task.contains(environment))
        .ok_or(EcsHelperVarietyError::NoSpecifiedTask(
          format!("Task specified in CLI does not exist, tasks you have: {task_names}.\nProject: {project}, application: {application}, environment: {environment}, cluster: {cluster}"))
        )?
  };

  Ok(task.to_owned())
}

pub async fn get_target_container(
  ecs_client: &EcsClient,
  config: &Config,
  cluster: &String,
  task_arn: &String,
  container_from_options: &Option<String>,
) -> miette::Result<String, EcsHelperVarietyError> {
  let Config {
    project,
    application,
    environment,
    ..
  } = config;

  let task = ecs_client.describe_task(task_arn, cluster).await?;
  let containers = task.containers.unwrap_or_default();
  let container_names = containers
    .iter()
    .map(|item| item.name.as_ref().unwrap().to_string())
    .collect::<Vec<_>>()
    .join(", ");

  let container = match container_from_options {
    Some(container_name) =>
      containers
        .iter()
        .find(|item| item.name.as_ref().unwrap().contains(container_name))
        .ok_or(EcsHelperVarietyError::NoSpecifiedContainer(format!(
          "Container specified in CLI does not exist, containers you have: {container_names}.\nProject: {project}, application: {application}, environment: {environment}, cluster: {cluster}, task: {task_arn}, container: {container_name}"
        )))?,
    None =>
      containers
        .iter()
        .find(|item| {
          let container_arn = item.name.as_ref().unwrap();
          container_arn.contains(application) && container_arn.contains(environment)
        })
        .ok_or(EcsHelperVarietyError::NoSpecifiedContainer(
          format!("222Container specified in CLI does not exist, containers you have: {container_names}.\nProject: {project}, application: {application}, environment: {environment}, cluster: {cluster}, task: {task_arn}"))
        )?
  };

  Ok(container.name.as_ref().unwrap().to_owned())
}
