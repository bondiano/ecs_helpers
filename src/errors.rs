use aws_sdk_ecr::{
  error::SdkError,
  operation::{
    describe_images::DescribeImagesError, describe_repositories::DescribeRepositoriesError,
    get_authorization_token::GetAuthorizationTokenError,
  },
};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum EcsHelperVarietyError {
  #[error("Failed to extract commit sha")]
  #[diagnostic(code(ecs_helper::config::extract_commit_sha_error))]
  ExtractCommitShaError,

  #[error("Failed to extract environment")]
  #[diagnostic(code(ecs_helper::config::extract_environment_error))]
  ExtractEnvironmentError,

  #[error(transparent)]
  #[diagnostic(code(ecs_helper::write::io_error))]
  IoError(#[from] std::io::Error),

  #[error("Failed to read command output: {0}")]
  #[diagnostic(code(ecs_helper::auth::read_output_error))]
  ReedOutputError(std::io::Error),

  #[error("Failed to get region")]
  #[diagnostic(code(ecs_helper::auth::get_region_error))]
  GetRegionError,

  #[error("Failed to get token:\n{0}")]
  #[diagnostic(code(ecs_helper::auth::get_token_error))]
  GetTokenError(SdkError<GetAuthorizationTokenError>),

  #[error("Failed to extract token")]
  #[diagnostic(code(ecs_helper::auth::extract_token_error))]
  ExtractTokenError,

  #[error("Failed to parse token:\n{0}")]
  #[diagnostic(code(ecs_helper::auth::parse_token_error))]
  ParseTokenError(#[from] base64::DecodeError),

  #[error("Failed to parse token:\n{0}")]
  #[diagnostic(code(ecs_helper::auth::parse_token_from_utf8_error))]
  ParseTokenFromUtf8Error(#[from] std::string::FromUtf8Error),

  #[error("Login command was failed\nWith status: {0}")]
  #[diagnostic(code(ecs_helper::login::login_failed))]
  LoginFailed(String),

  #[error("Failed to describe repositories:\n{0}")]
  #[diagnostic(code(ecs_helper::ecr::describe_repositories_error))]
  DescribeRepositoriesError(SdkError<DescribeRepositoriesError>),

  #[error("Describe images was failed:\n{0}")]
  #[diagnostic(code(ecs_helper::ecr::describe_images_error))]
  DescribeImagesError(SdkError<DescribeImagesError>),
}
