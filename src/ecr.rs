use aws_config::SdkConfig;
use aws_sdk_ecr::{
  types::{ImageDetail, ImageIdentifier, Repository},
  Client,
};
use base64::{alphabet, engine, Engine};

use crate::errors::EcsHelperVarietyError;

pub struct EcrClient {
  client: Client,
}

impl EcrClient {
  pub fn new(config: &SdkConfig) -> Self {
    Self {
      client: Client::new(config),
    }
  }

  pub async fn get_private_repositories(
    &self,
  ) -> miette::Result<Vec<Repository>, EcsHelperVarietyError> {
    let response = self
      .client
      .describe_repositories()
      .send()
      .await
      .map_err(EcsHelperVarietyError::DescribeRepositoriesError)?;

    Ok(response.repositories().to_vec())
  }

  pub async fn describe_images(
    &self,
    repository_name: &str,
    image_id: ImageIdentifier,
  ) -> miette::Result<ImageDetail, EcsHelperVarietyError> {
    let response = self
      .client
      .describe_images()
      .repository_name(repository_name)
      .image_ids(image_id)
      .send()
      .await
      .map_err(EcsHelperVarietyError::DescribeImagesError)?;

    let image_details = response
      .image_details()
      .first()
      .ok_or(EcsHelperVarietyError::ExtractImageError)?;

    Ok(image_details.to_owned())
  }

  pub async fn get_token(&self) -> miette::Result<String, EcsHelperVarietyError> {
    let auth_token_data = self
      .client
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
}
