use crate::{config::Config, ecs::EcsClient, errors::EcsHelperVarietyError};

pub async fn get_current_service(
  ecs_client: &EcsClient,
  config: &Config,
  cluster: &String,
  service_from_options: &Option<String>,
) -> miette::Result<String, EcsHelperVarietyError> {
  let project = &config.project;
  let application = &config.application;
  let environment = &config.environment;

  let services = ecs_client.get_services(cluster).await?;
  let service_names = services.join(", ");

  let service_name = match service_from_options {
    Some(service_name) => {
      let service_name = services
        .iter()
        .find(|service| service.contains(service_name))
        .ok_or(EcsHelperVarietyError::NoSpecifiedService(format!(
          "Service specified in cli not exists, services you have: {service_names}.\nProject: {project}, application: {application}, environment: {environment}, cluster: {cluster}, service: {service_name}"
        )))?;

      service_name.to_owned()
    }
    None => {
      let service_name = services
        .iter()
        .find(|service| service.contains(application) && service.contains(environment))
        .ok_or(EcsHelperVarietyError::NoSpecifiedService(
          format!("Service specified in cli not exists, services you have: {service_names}.\nProject: {project}, application: {application}, environment: {environment}, cluster: {cluster}"))
        )?;

      service_name.to_owned()
    }
  };

  Ok(service_name)
}
