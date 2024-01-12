use ecs_helpers::{
  args::ExportImagesArguments, config::Config, ecr::EcrClient, errors::EcsHelperVarietyError,
  Command,
};
use regex::Regex;

#[derive(Debug)]
pub struct ExportImagesCommand {
  config: Config,
}

impl ExportImagesCommand {
  pub fn new(config: Config, _: ExportImagesArguments) -> Self {
    Self { config }
  }
}

impl Command for ExportImagesCommand {
  fn name(&self) -> String {
    "export_images".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let version = &self.config.version;
    let project = &self.config.project;
    let application = &self.config.application;
    let sdk_config = &self.config.sdk_config;

    let ecr_client = EcrClient::new(sdk_config);

    let private_repositories = ecr_client.get_private_repositories().await?;
    let private_repositories_entries = private_repositories
      .iter()
      .filter_map(|repo| {
        let repository_name = repo.repository_name()?;

        let pattern = format!("{}-{}-(.*)", project, application);
        let re = Regex::new(&pattern).unwrap();

        let container_name = re.captures(repository_name);
        let container_name = container_name.as_ref()?;

        let container_name = container_name.get(1)?.as_str();

        let key = container_name.to_uppercase().replace('-', "_") + "_IMAGE";

        let repository_uri = repo.repository_uri()?;
        let value = format!("{repository_uri}:{version}");

        Some(format!("{}={}", key, value))
      })
      .collect::<Vec<String>>();

    let variables = private_repositories_entries.join(" ");

    println!("export {}", variables);

    Ok(())
  }
}
