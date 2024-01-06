use crate::errors::EcsHelperVarietyError;
use git2::Repository;

#[derive(Debug)]
pub struct Config {
  pub environment: String,
  pub version: String,
}

impl Config {
  pub async fn new(environment: Option<String>) -> miette::Result<Self, EcsHelperVarietyError> {
    let commit_sha = extract_commit_sha()?;
    let need_use_image_tag_env_prefix = false;
    let environment = extract_environment(environment)?;
    let version = extract_version(commit_sha, &environment, need_use_image_tag_env_prefix);

    Ok(Self {
      version,
      environment,
    })
  }
}

fn extract_version(
  commit_sha: String,
  environment: &String,
  need_use_image_tag_env_prefix: bool,
) -> String {
  if need_use_image_tag_env_prefix {
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

fn extract_environment_from_env() -> miette::Result<String, EcsHelperVarietyError> {
  match std::env::var("ENVIRONMENT") {
    Ok(environment) => Ok(environment),
    Err(_) => extract_environment_from_branch_name(),
  }
}

fn extract_environment(
  environment: Option<String>,
) -> miette::Result<String, EcsHelperVarietyError> {
  match environment {
    Some(environment) => Ok(environment),
    None => extract_environment_from_env(),
  }
}
