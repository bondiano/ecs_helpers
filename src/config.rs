use crate::{args::CommandArguments, errors::EcsHelperVarietyError};
use aws_config::{Region, SdkConfig};
use git2::Repository;

const DEFAULT_REGION: &str = "us-east-1";

#[derive(Debug, Clone)]
pub struct Config {
  pub environment: String,
  pub version: String,
  pub project: String,
  pub application: String,
  pub region: Region,
  pub sdk_config: SdkConfig,
  pub aws_account_id: String,
}

impl Config {
  pub async fn new(args: &CommandArguments) -> miette::Result<Self, EcsHelperVarietyError> {
    let sdk_config = aws_config::load_from_env().await;

    let commit_sha = Config::extract_commit_sha()?;
    let environment = Config::extract_environment(&args.environment)?;
    let version = args.version.to_owned().unwrap_or(Config::extract_version(
      args.use_image_tag_env_prefix,
      commit_sha,
      &environment,
    ));
    // TODO: add aws_account_id detection and errors
    let aws_account_id = args.aws_account_id.to_owned().unwrap_or("".to_string());
    let project = args.project.to_owned();
    // TODO: add application detection and errors
    let application = args.application.to_owned().unwrap_or("".to_string());
    let region = sdk_config
      .region()
      .unwrap_or(&Region::new(DEFAULT_REGION))
      .to_owned();

    Ok(Self {
      region,
      sdk_config,
      application,
      version,
      environment,
      project,
      aws_account_id,
    })
  }

  fn extract_version(
    use_image_tag_env_prefix: bool,
    commit_sha: String,
    environment: &String,
  ) -> String {
    if use_image_tag_env_prefix {
      format!("{}-{}", environment, commit_sha)
    } else {
      commit_sha
    }
  }

  fn extract_commit_sha() -> miette::Result<String, EcsHelperVarietyError> {
    if let Ok(commit_sha) = std::env::var("CI_COMMIT_SHA") {
      Ok(commit_sha)
    } else {
      let repo = Repository::open(".").map_err(|_| EcsHelperVarietyError::ExtractCommitShaError)?;

      let rev_spec = repo
        .revparse_single("HEAD")
        .map_err(|_| EcsHelperVarietyError::ExtractCommitShaError)?;

      Ok(rev_spec.id().to_string())
    }
  }

  fn extract_environment_from_branch_name() -> miette::Result<String, EcsHelperVarietyError> {
    let repo = Repository::open(".").map_err(|_| EcsHelperVarietyError::ExtractEnvironmentError)?;

    let branch = repo
      .head()
      .map_err(|_| EcsHelperVarietyError::ExtractEnvironmentError)?;
    let branch = branch
      .shorthand()
      .ok_or(EcsHelperVarietyError::ExtractEnvironmentError)?;

    let environment = match branch {
      "master" => "production",
      "main" => "production",
      "qa" => "qa",
      "uat" => "uat",
      "staging" => "staging",
      "demo" => "demo",
      _ => Err(EcsHelperVarietyError::ExtractEnvironmentError)?,
    };

    Ok(environment.to_string())
  }

  fn extract_environment(
    environment: &Option<String>,
  ) -> miette::Result<String, EcsHelperVarietyError> {
    match environment {
      Some(environment) => Ok(environment.clone()),
      None => Config::extract_environment_from_branch_name(),
    }
  }
}
