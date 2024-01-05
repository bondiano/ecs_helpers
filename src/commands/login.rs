use ecs_helpers::{auth, errors::EcsHelperVarietyError};

use aws_config::{Region, SdkConfig};

#[derive(Debug)]
pub struct LoginCommandOptions {
  pub aws_account_id: String,
  pub region: Region,
  pub config: SdkConfig,
}

impl LoginCommandOptions {
  pub async fn new(aws_account_id: String) -> miette::Result<Self> {
    let config = aws_config::load_from_env().await;
    let region = config
      .region()
      .ok_or(EcsHelperVarietyError::GetRegionError)?
      .clone();

    Ok(Self {
      config,
      region,
      aws_account_id,
    })
  }
}

pub async fn login(options: LoginCommandOptions) -> miette::Result<(), EcsHelperVarietyError> {
  let auth_output = auth::login_to_ecr(
    &options.config,
    options.region.to_string(),
    options.aws_account_id,
  )
  .await?;

  if auth_output.status.success() {
    Ok(())
  } else {
    Err(EcsHelperVarietyError::LoginFailed(
      auth_output.status.to_string(),
    ))
  }
}
