use crate::{config::Config, ecs::EcsClient, errors::EcsHelperVarietyError};

pub async fn get_current_cluster(
  ecs_client: &EcsClient,
  config: &Config,
  cluster_from_options: &Option<String>,
) -> miette::Result<String, EcsHelperVarietyError> {
  let Config {
    project,
    environment,
    ..
  } = config;

  let clusters = ecs_client.get_clusters().await?;
  let cluster_names = clusters.join(", ");

  let cluster_name = match cluster_from_options {
    Some(cluster_name) =>
       clusters
        .iter()
        .find(|cluster| cluster.contains(cluster_name))
        .ok_or(EcsHelperVarietyError::NoSpecifiedCluster(format!(
          "Cluster specified in cli not exists, clusters you have: {cluster_names}.\nProject: {project}, environment: {environment}, cluster: {cluster_name}"
        )))?,
    None =>
       clusters
        .iter()
        .find(|cluster| cluster.contains(&config.project) && cluster.contains(&config.environment))
        .ok_or(EcsHelperVarietyError::NoSpecifiedCluster(
          format!("Cluster specified in cli not exists, clusters you have: {cluster_names}.\nProject: {project}, environment: {environment}"))
        )?
  };

  Ok(cluster_name.to_owned())
}
