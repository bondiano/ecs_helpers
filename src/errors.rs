use aws_sdk_ecr::{
  error::SdkError,
  operation::{
    describe_images::DescribeImagesError, describe_repositories::DescribeRepositoriesError,
    get_authorization_token::GetAuthorizationTokenError,
  },
};
use aws_sdk_ecs::operation::{
  describe_services::DescribeServicesError, describe_task_definition::DescribeTaskDefinitionError,
  describe_tasks::DescribeTasksError, list_clusters::ListClustersError,
  list_services::ListServicesError, list_task_definitions::ListTaskDefinitionsError,
  list_tasks::ListTasksError, register_task_definition::RegisterTaskDefinitionError,
  run_task::RunTaskError,
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
  GetTokenError(#[from] SdkError<GetAuthorizationTokenError>),

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
  DescribeRepositoriesError(#[from] SdkError<DescribeRepositoriesError>),

  #[error("Describe images was failed:\n{0}")]
  #[diagnostic(code(ecs_helper::ecr::describe_images_error))]
  DescribeImagesError(#[from] SdkError<DescribeImagesError>),

  #[error("Failed to extract image")]
  #[diagnostic(code(ecs_helper::ecr::extract_image_error))]
  ExtractImageError,

  #[error("Failed to describe task definition:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::describe_task_definition_error))]
  DescribeTaskDefinitionError(#[from] SdkError<DescribeTaskDefinitionError>),

  #[error("Failed to extract description")]
  #[diagnostic(code(ecs_helper::ecs::extract_description_error))]
  ExtractTaskDefinitionError,

  #[error("Failed to get list task definitions:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::get_list_task_definitions_error))]
  GetListTaskDefinitionsError(#[from] SdkError<ListTaskDefinitionsError>),

  #[error("Failed to register task definition:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::no_task_definitions_found))]
  RegisterTaskDefinitionError(#[from] SdkError<RegisterTaskDefinitionError>),

  #[error("Failed to get list clusters:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::get_list_clusters_error))]
  GetListClustersError(#[from] SdkError<ListClustersError>),

  #[error("No clusters found")]
  #[diagnostic(code(ecs_helper::ecs::no_clusters_found))]
  NoClustersFound,

  #[error("Cluster specified in cli not exists, clusters you have:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::no_specified_cluster))]
  NoSpecifiedCluster(String),

  #[error("Service specified in cli not exists, services you have:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::no_specified_service))]
  NoSpecifiedService(String),

  #[error("Failed to run task:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::run_task_error))]
  RunTaskError(#[from] SdkError<RunTaskError>),

  #[error("No tasks found")]
  #[diagnostic(code(ecs_helper::ecs::no_tasks_found))]
  NoTasksFound,

  #[error("Failed to get list services:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::get_list_services_error))]
  GetListServicesError(#[from] SdkError<ListServicesError>),

  #[error("Failed to describe services:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::describe_services_error))]
  DescribeServiceError(#[from] SdkError<DescribeServicesError>),

  #[error("No services found")]
  #[diagnostic(code(ecs_helper::ecs::no_services_found))]
  NoServicesFound,

  #[error("Failed to get list tasks:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::get_list_tasks_error))]
  GetListTasksError(#[from] SdkError<ListTasksError>),

  #[error("Failed to describe task:\n{0}")]
  #[diagnostic(code(ecs_helper::ecs::describe_task_error))]
  DescribeTaskError(#[from] SdkError<DescribeTasksError>),

  #[error("Task run timeout ({0})")]
  #[diagnostic(code(ecs_helper::ecs::wait_task_timeout_error))]
  WaitTaskTimeoutError(u64),

  #[error("Task {task_arn} was failed with code {code}")]
  #[diagnostic(code(ecs_helper::ecs::task_was_failed))]
  TaskWasFailed { task_arn: String, code: i32 },
}
