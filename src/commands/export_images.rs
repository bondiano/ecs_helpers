use aws_config::SdkConfig;
use ecs_helpers::{config::Config, ecr::EcrClient, errors::EcsHelperVarietyError};
use regex::Regex;

#[derive(Debug)]
pub struct ExportImagesCommandOptions {
  pub sdk_config: SdkConfig,
  pub config: Config,
  pub project: String,
  pub application: String,
}

impl ExportImagesCommandOptions {
  pub async fn new(config: Config, project: String, application: String) -> miette::Result<Self> {
    let sdk_config = aws_config::load_from_env().await;

    Ok(Self {
      sdk_config,
      config,
      project,
      application,
    })
  }
}

pub async fn export_images(
  options: ExportImagesCommandOptions,
) -> miette::Result<String, EcsHelperVarietyError> {
  let ecr_client = EcrClient::new(&options.sdk_config);

  let private_repositories = ecr_client.get_private_repositories().await?;
  let private_repositories_entries = private_repositories
    .iter()
    .filter_map(|repo| {
      let repository_name = repo.repository_name()?;

      let pattern = format!("{}-{}-(.*)", options.project, options.application);
      let re = Regex::new(&pattern).unwrap();

      let container_name = re.captures(repository_name);
      let container_name = container_name.as_ref()?;

      let container_name = container_name.get(1)?.as_str();

      let key = container_name.to_uppercase().replace('-', "_") + "_IMAGE";

      let repository_uri = repo.repository_uri()?;
      let value = format!("{}:{}", repository_uri, options.config.version);

      Some(format!("{}={}", key, value))
    })
    .collect::<Vec<String>>();

  let variables_array = Vec::from(["export".to_string()]);
  let variables_array = variables_array
    .into_iter()
    .chain(private_repositories_entries.into_iter())
    .collect::<Vec<String>>();

  Ok(variables_array.join(" "))
}
