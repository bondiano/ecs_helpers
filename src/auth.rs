use std::process::{Output, Stdio};

use aws_config::{Region, SdkConfig};
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{ecr::EcrClient, errors::EcsHelperVarietyError};

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
  let ecr_client = EcrClient::new(&sdk_config);
  let token = ecr_client.get_token().await?;

  run_docker_login(account_id, region, &token).await
}
