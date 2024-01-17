use crate::{args::CommandArguments, errors::EcsHelperVarietyError};
use aws_config::{Region, SdkConfig};
use git2::Repository;

const DEFAULT_REGION: &str = "us-east-1";

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
    let aws_account_id = args
      .aws_account_id
      .to_owned()
      .unwrap_or(Config::extract_aws_account_id(&sdk_config).await);
    let project = args.project.to_owned();
    let application = args.application.to_owned();
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

  async fn extract_aws_account_id(sdk_config: &SdkConfig) -> String {
    let sts_client = aws_sdk_sts::Client::new(sdk_config);
    let caller_identity = sts_client.get_caller_identity().send().await;

    let account_id = match caller_identity {
      Ok(caller_identity) => caller_identity.account,
      Err(_) => {
        log::warn!("Unable to get AWS account ID, using empty string");
        return "".to_string();
      }
    };

    match account_id {
      Some(account_id) => account_id,
      None => {
        log::warn!("No AWS account ID provided, using empty string");
        "".to_string()
      }
    }
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
      let repo = Repository::open(".")
        .map_err(|err| EcsHelperVarietyError::ExtractCommitShaError(err.to_string()))?;

      let rev_spec = repo
        .revparse_single("HEAD")
        .map_err(|err| EcsHelperVarietyError::ExtractCommitShaError(err.to_string()))?;

      Ok(rev_spec.id().to_string())
    }
  }

  fn extract_environment_from_branch_name() -> miette::Result<String, EcsHelperVarietyError> {
    let repo = Repository::open(".")
      .map_err(|err| EcsHelperVarietyError::ExtractEnvironmentError(err.to_string()))?;

    let branch = repo
      .head()
      .map_err(|err| EcsHelperVarietyError::ExtractEnvironmentError(err.to_string()))?;
    let branch = branch
      .name()
      .ok_or(EcsHelperVarietyError::ExtractEnvironmentError(
        "Could not extract branch name.".to_string(),
      ))?;
    let branch = branch
      .split('/')
      .last()
      .ok_or(EcsHelperVarietyError::ExtractEnvironmentError(
        format!("Could not extract branch name from {branch}.")
      ))?;

    let environment = match branch {
      "master" => "production",
      "main" => "production",
      "qa" => "qa",
      "uat" => "uat",
      "staging" => "staging",
      "demo" => "demo",
      _ => Err(EcsHelperVarietyError::ExtractEnvironmentError(format!(
        "Could not match branch name {branch} with environment."
      )))?,
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

#[cfg(test)]
mod tests {
  use super::*;
  use git2::{Repository, RepositoryInitOptions};
  use sealed_test::prelude::*;

  #[test]
  fn test_extract_version() {
    let commit_sha = "1234567890".to_string();
    let environment = "production".to_string();

    let version = Config::extract_version(false, commit_sha.clone(), &environment);
    assert_eq!(version, commit_sha);

    let version = Config::extract_version(true, commit_sha.clone(), &environment);
    assert_eq!(version, format!("{}-{}", environment, commit_sha));
  }

  #[sealed_test]
  fn test_extract_commit_sha() {
    let expected_commit_sha = "1234567890";
    std::env::set_var("CI_COMMIT_SHA", expected_commit_sha);

    let commit_sha = Config::extract_commit_sha().unwrap();
    assert_eq!(commit_sha, expected_commit_sha);
  }

  #[sealed_test]
  fn test_extract_environment_from_branch_name() {
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main");
    let repo = Repository::init_opts(".", &opts).unwrap();
    {
      let mut config = repo.config().unwrap();
      config.set_str("user.name", "name").unwrap();
      config.set_str("user.email", "email").unwrap();
      let mut index = repo.index().unwrap();
      let id = index.write_tree().unwrap();

      let tree = repo.find_tree(id).unwrap();
      let sig = repo.signature().unwrap();
      repo
        .commit(Some("HEAD"), &sig, &sig, "initial\n\nbody", &tree, &[])
        .unwrap();
    }

    let environment = Config::extract_environment_from_branch_name().unwrap();
    assert_eq!(environment, "production");

    repo
      .branch("qa", &repo.head().unwrap().peel_to_commit().unwrap(), false)
      .unwrap();
    repo.set_head("refs/heads/qa").unwrap();
    let environment = Config::extract_environment_from_branch_name().unwrap();
    assert_eq!(environment, "qa");

    repo
      .branch(
        "uat",
        &repo.head().unwrap().peel_to_commit().unwrap(),
        false,
      )
      .unwrap();
    repo.set_head("refs/heads/uat").unwrap();
    let environment = Config::extract_environment_from_branch_name().unwrap();
    assert_eq!(environment, "uat");

    repo
      .branch(
        "staging",
        &repo.head().unwrap().peel_to_commit().unwrap(),
        false,
      )
      .unwrap();
    repo.set_head("refs/heads/staging").unwrap();
    let environment = Config::extract_environment_from_branch_name().unwrap();
    assert_eq!(environment, "staging");

    repo
      .branch(
        "demo",
        &repo.head().unwrap().peel_to_commit().unwrap(),
        false,
      )
      .unwrap();
    repo.set_head("refs/heads/demo").unwrap();
    let environment = Config::extract_environment_from_branch_name().unwrap();
    assert_eq!(environment, "demo");
  }
}
