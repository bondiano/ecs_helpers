use aws_sdk_ecr::{
  error::SdkError, operation::get_authorization_token::GetAuthorizationTokenError,
};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum EcsHelperVarietyError {
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
}
