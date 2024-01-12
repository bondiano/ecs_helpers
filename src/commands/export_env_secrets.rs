use ecs_helpers::{
  args::ExportEnvSecretsCommandArguments, config::Config, errors::EcsHelperVarietyError,
  ssm::SSMClient, Command,
};

#[derive(Debug)]
pub struct ExportEnvSecretsCommand {
  config: Config,
  variables_to_export: Option<Vec<String>>,
  ssm_client: SSMClient,
}

impl ExportEnvSecretsCommand {
  pub fn new(config: Config, args: ExportEnvSecretsCommandArguments) -> Self {
    let ssm_client = SSMClient::new(&config.sdk_config);

    Self {
      config,
      ssm_client,
      variables_to_export: args.name,
    }
  }
}

impl Command for ExportEnvSecretsCommand {
  fn name(&self) -> String {
    "export_env_secrets".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let variables_to_export = self
      .variables_to_export
      .clone()
      .ok_or(EcsHelperVarietyError::NoEnvVariablesToExport)?;

    let params_name = variables_to_export
      .iter()
      .map(|var_name| {
        format!(
          "/{}-{}-{}/{}",
          self.config.project, self.config.application, self.config.environment, var_name
        )
      })
      .collect::<Vec<String>>();

    let aws_ssm_params = self.ssm_client.get_parameters(params_name, true).await?;

    let variables = aws_ssm_params
      .iter()
      .filter_map(|aws_ssm_param| {
        let value = aws_ssm_param.value()?;
        let name = aws_ssm_param.name()?;

        Some(format!("{}={}", name, value))
      })
      .collect::<Vec<String>>()
      .join(" ");

    println!("export {}", variables);

    Ok(())
  }
}
