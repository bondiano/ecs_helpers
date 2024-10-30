use aws_config::SdkConfig;
use aws_sdk_ecr::{
  types::{ImageDetail, ImageIdentifier, Repository},
  Client,
};
use aws_sdk_ecs::types::ContainerDefinition;
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

  /// Extracts token from AWS ECR.
  ///
  /// Doing the same as `aws ecr get-login-password --region=` but without aws cli.
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

  /// Creates new container definition from existing container definition.
  ///
  /// Set image to `repository_uri:version`, `repository_uri` is extracted from existing container definition.
  pub async fn create_new_container_definition_from(
    &self,
    container_definition: &ContainerDefinition,
    repositories: Vec<Repository>,
    version: &String,
  ) -> miette::Result<ContainerDefinition, EcsHelperVarietyError> {
    let repository = repositories
      .iter()
      .find(|repository| {
        let repository_uri = match repository.repository_uri() {
          Some(repository_uri) => repository_uri,
          None => return false,
        };

        let image = match container_definition.image() {
          Some(image) => image,
          None => return false,
        };

        image.contains(repository_uri)
      })
      .unwrap();

    let repository_name = repository.repository_name().unwrap();
    let repository_uri = repository.repository_uri().unwrap();

    let ecr_base = repository_uri.split('/').collect::<Vec<_>>();
    let ecr_base = ecr_base.first().unwrap();

    if !container_definition.image().unwrap().contains(ecr_base) {
      return Err(EcsHelperVarietyError::ContainerDefinitionImageError(
        container_definition.image().unwrap().to_owned(),
      ));
    };

    let image_identifier = ImageIdentifier::builder().image_tag(version).build();

    self
      .describe_images(repository_name, image_identifier)
      .await?;

    let mut new_container_definition = container_definition.clone();

    // we're partially cloning container definition because we need to change image according to repository
    new_container_definition.image = Some(format!("{repository_uri}:{version}"));

    Ok(new_container_definition)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use aws_config::{BehaviorVersion, Region};
  use aws_sdk_ecr::config::{Credentials, SharedCredentialsProvider};
  use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
  use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
  use aws_smithy_types::body::SdkBody;

  #[tokio::test]
  async fn test_describe_images() {
    let request = HttpRequest::new(SdkBody::from(""));

    let response = http::Response::builder()
      .status(200)
      .body(SdkBody::from(
        "
         {
            \"imageDetails\": [
              {
                \"imageDigest\": \"sha256:1234567890\",
                \"imageTags\": [\"latest\"],
                \"registryId\": \"1234567890\",
                \"repositoryName\": \"test\",
                \"imageSizeInBytes\": 1234567890,
                \"imagePushedAt\": 1234567890,
                \"imageScanFindingsSummary\": {
                  \"findingSeverityCounts\": {
                    \"CRITICAL\": 0,
                    \"HIGH\": 0,
                    \"MEDIUM\": 0,
                    \"LOW\": 0,
                    \"INFORMATIONAL\": 0,
                    \"UNDEFINED\": 0
                  },
                  \"imageScanCompletedAt\": 1234567890,
                  \"vulnerabilitySourceUpdatedAt\": 1234567890
                },
                \"imageScanStatus\": {
                  \"status\": \"COMPLETE\",
                  \"description\": \"Image scan completed and found no vulnerabilities\"
                }
              }
            ]
         }
        ",
      ))
      .unwrap();

    let page = ReplayEvent::new(request, response);

    let http_client = StaticReplayClient::new(vec![page]);
    let credentials = SharedCredentialsProvider::new(Credentials::for_tests_with_session_token());
    let sdk_config = SdkConfig::builder()
      .region(Region::new("us-east-1"))
      .behavior_version(BehaviorVersion::latest())
      .credentials_provider(credentials)
      .http_client(http_client)
      .build();

    let client = EcrClient::new(&sdk_config);
    let image_id = ImageIdentifier::builder().image_tag("latest").build();
    let image_detail = client.describe_images("test", image_id).await.unwrap();

    assert_eq!(image_detail.image_digest(), Some("sha256:1234567890"));
  }
}
