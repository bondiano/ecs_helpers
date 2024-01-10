use ecs_helpers::{
  args::LoginCommandArguments, auth, config::Config, errors::EcsHelperVarietyError, Command,
};

#[derive(Debug)]
pub struct LoginCommand {
  aws_account_id: String,
  config: Config,
}

impl LoginCommand {
  pub fn new(config: Config, arguments: LoginCommandArguments) -> Self {
    Self {
      config,
      aws_account_id: arguments.aws_account_id,
    }
  }
}

impl Command for LoginCommand {
  fn name(&self) -> String {
    "login".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let sdk_config = &self.config.sdk_config;
    let region = &self.config.region;

    let auth_output = auth::login_to_ecr(sdk_config, region, &self.aws_account_id).await?;

    if auth_output.status.success() {
      Ok(())
    } else {
      Err(EcsHelperVarietyError::LoginFailed(
        auth_output.status.to_string(),
      ))
    }
  }
}
