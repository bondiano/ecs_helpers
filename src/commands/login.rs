use ecs_helpers::{
  args::LoginCommandArguments, auth, config::Config, errors::EcsHelperVarietyError, Command,
};

pub struct LoginCommand {
  config: Config,
}

impl LoginCommand {
  pub fn new(config: Config, _: LoginCommandArguments) -> Self {
    Self { config }
  }
}

impl Command for LoginCommand {
  fn name(&self) -> String {
    "login".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let Config {
      sdk_config,
      region,
      aws_account_id,
      ..
    } = &self.config;

    let auth_output = auth::login_to_ecr(sdk_config, region, aws_account_id).await?;

    if auth_output.status.success() {
      log::info!("Login succeeded!");

      Ok(())
    } else {
      Err(EcsHelperVarietyError::LoginFailed(format!(
        "Login failed with status code: {}",
        auth_output.status.code().unwrap_or(0)
      )))
    }
  }
}
