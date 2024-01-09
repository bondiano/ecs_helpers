use crate::{config::Config, ecs::EcsClient, errors::EcsHelperVarietyError};

pub async fn get_current_cluster(
  ecs_client: &EcsClient,
  config: &Config,
  cluster_from_options: &Option<String>,
) -> miette::Result<String, EcsHelperVarietyError> {
  let project = &config.project;
  let environment = &config.environment;

  let clusters = ecs_client.get_clusters().await?;
  let cluster_names = clusters.join(", ");

  let cluster_name = match cluster_from_options {
    Some(cluster_name) => {
      let cluster_name = clusters
        .iter()
        .find(|cluster| cluster.contains(cluster_name))
        .ok_or(EcsHelperVarietyError::NoSpecifiedCluster(format!(
          "Cluster specified in cli not exists, clusters you have: {cluster_names}.\nProject: {project}, environment: {environment}, cluster: {cluster_name}"
        )))?;

      cluster_name.to_owned()
    }
    None => {
      let cluster_name = clusters
        .iter()
        .find(|cluster| cluster.contains(&config.project) && cluster.contains(&config.environment))
        .ok_or(EcsHelperVarietyError::NoSpecifiedCluster(
          format!("Cluster specified in cli not exists, clusters you have: {cluster_names}.\nProject: {project}, environment: {environment}"))
        )?;

      cluster_name.to_owned()
    }
  };

  Ok(cluster_name)
}
