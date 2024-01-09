use aws_config::SdkConfig;
use aws_sdk_ecr::{
  types::{ImageDetail, ImageIdentifier, Repository},
  Client,
};

use crate::errors::EcsHelperVarietyError;

#[derive(Debug, Clone)]
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
}
