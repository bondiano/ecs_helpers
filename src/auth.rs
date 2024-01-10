use std::process::{Output, Stdio};

use aws_config::{Region, SdkConfig};
use aws_sdk_ecr::Client as EcrClient;
use base64::{alphabet, engine, Engine};
use tokio::{io::AsyncWriteExt, process::Command};

use crate::errors::EcsHelperVarietyError;

async fn get_token(config: &SdkConfig) -> miette::Result<String, EcsHelperVarietyError> {
  let ecr_client = EcrClient::new(config);
  let auth_token_data = ecr_client
    .get_authorization_token()
    .send()
    .await
    .map_err(EcsHelperVarietyError::GetTokenError)?;
  let auth_data = auth_token_data
    .authorization_data()
    .first()
    .ok_or(EcsHelperVarietyError::ExtractTokenError)?;

  let base_64_engine =
    engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::PAD);

  let token = auth_data
    .authorization_token()
    .ok_or(EcsHelperVarietyError::ExtractTokenError)?;
  let token = base_64_engine
    .decode(token)
    .map_err(EcsHelperVarietyError::ParseTokenError)?;
  let token = token.as_slice();
  let token =
    String::from_utf8(token.to_vec()).map_err(EcsHelperVarietyError::ParseTokenFromUtf8Error)?;
  let token = token.split(':').collect::<Vec<&str>>();
  let token = token
    .get(1)
    .ok_or(EcsHelperVarietyError::ExtractTokenError)?;

  Ok(token.to_string())
}

async fn run_docker_login(
  account_id: &String,
  region: &Region,
  token: &String,
) -> miette::Result<Output, EcsHelperVarietyError> {
  let mut child = Command::new("docker")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .arg("login")
    .arg("-u")
    .arg("AWS")
    .arg("--password-stdin")
    .arg(format!("{account_id}.dkr.ecr.{region}.amazonaws.com"))
    .spawn()
    .map_err(EcsHelperVarietyError::ReedOutputError)?;

  if let Some(child_stdin) = child.stdin.as_mut() {
    child_stdin
      .write_all(token.as_bytes())
      .await
      .map_err(EcsHelperVarietyError::ReedOutputError)?;
  }

  child
    .wait_with_output()
    .await
    .map_err(EcsHelperVarietyError::ReedOutputError)
}

pub async fn login_to_ecr(
  sdk_config: &SdkConfig,
  region: &Region,
  account_id: &String,
) -> miette::Result<Output, EcsHelperVarietyError> {
  let token = get_token(sdk_config).await?;

  run_docker_login(account_id, region, &token).await
}
