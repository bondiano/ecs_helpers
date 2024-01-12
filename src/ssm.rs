use aws_config::SdkConfig;
use aws_sdk_ssm::{types::Parameter, Client};

use crate::errors::EcsHelperVarietyError;

#[derive(Debug, Clone)]
pub struct SSMClient {
  client: Client,
}

impl SSMClient {
  pub fn new(config: &SdkConfig) -> Self {
    Self {
      client: Client::new(config),
    }
  }

  pub async fn get_parameters(
    &self,
    names: Vec<String>,
    with_decryption: bool,
  ) -> miette::Result<Vec<Parameter>, EcsHelperVarietyError> {
    let mut request = self
      .client
      .get_parameters()
      .with_decryption(with_decryption);

    for name in names {
      request = request.names(name);
    }

    let response = request
      .send()
      .await
      .map_err(EcsHelperVarietyError::GetSSMParametersError)?;

    let parameters = response.parameters().to_vec();

    Ok(parameters)
  }
}
