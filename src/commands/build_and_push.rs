use ecs_helpers::{
  args::BuildAndPushCommandArguments, auth, config::Config, ecr::EcrClient,
  errors::EcsHelperVarietyError, Command,
};
use futures::try_join;
use tokio::process::Command as TokioCommand;

pub struct BuildAndPushCommand {
  config: Config,
  ecr_client: EcrClient,
  image: String,
  should_cache: bool,
  build_arg: Option<Vec<String>>,
  directory: String,
  file: String,
  repository: Option<String>,
}

impl BuildAndPushCommand {
  pub fn new(config: Config, args: BuildAndPushCommandArguments) -> Self {
    let ecr_client = EcrClient::new(&config.sdk_config);

    Self {
      config,
      ecr_client,
      image: args.image,
      should_cache: args.cache,
      build_arg: args.build_arg,
      directory: args.directory,
      file: args.file,
      repository: args.repository,
    }
  }

  async fn get_repository(&self) -> miette::Result<String, EcsHelperVarietyError> {
    let repository = self.ecr_client.get_private_repositories().await?;

    let with_name = repository
      .iter()
      .filter_map(|repo| {
        let arn = repo.repository_arn()?;

        if arn.contains(&self.image) {
          Some(repo)
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    if with_name.len() == 1 {
      let repository = with_name[0].repository_uri().unwrap().to_string();
      return Ok(repository);
    }

    let exact = with_name
      .iter()
      .filter_map(|repo| {
        let arn = repo.repository_arn()?;

        if let Some(repository) = &self.repository {
          if arn.contains(repository) {
            return Some(repo);
          }
        }

        let same_project = arn.contains(&self.config.project);
        let same_application = arn.contains(&self.config.application);

        if same_project && same_application {
          Some(repo)
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    if exact.len() == 1 {
      let repository = exact[0].repository_uri().unwrap().to_string();
      return Ok(repository);
    }

    if exact.len() > 1 {
      return Err(EcsHelperVarietyError::MultipleRepositoriesFound(
        exact
          .iter()
          .map(|repo| repo.repository_name().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(", "),
      ));
    }

    Err(EcsHelperVarietyError::NoRepositoryFound)
  }

  async fn pull_image_to_cache(
    &self,
    repository: &String,
  ) -> miette::Result<(), EcsHelperVarietyError> {
    let latest_tag = format!("{}:latest", repository);

    let output = TokioCommand::new("docker")
      .arg("pull")
      .arg(latest_tag)
      .output()
      .await?;

    if !output.status.success() {
      return Err(EcsHelperVarietyError::PullImageError(String::from_utf8(
        output.stderr,
      )?));
    }

    Ok(())
  }

  async fn build(&self, repository: &String) -> miette::Result<(), EcsHelperVarietyError> {
    let mut command = TokioCommand::new("docker");
    command.arg("build");
    command.arg(self.directory.clone());

    command.arg(format!("--file={}", self.file.clone()));

    if let Some(build_arg) = &self.build_arg {
      for build_arg in build_arg {
        command.arg(format!("--build-arg={}", build_arg));
      }
    }

    if self.should_cache {
      command.arg("--cache-from");
      command.arg(format!("{}:latest", repository));
    }

    let latest_tag: String = format!("{}:latest", repository);
    let version_tag: String = format!("{}:{}", repository, self.config.version);
    command.arg("-t");
    command.arg(&version_tag);
    command.arg("-t");
    command.arg(&latest_tag);

    let output = command.output().await?;

    log::info!("Building with two tags: {} & {}", latest_tag, version_tag);

    if !output.status.success() {
      return Err(EcsHelperVarietyError::BuildImageError(String::from_utf8(
        output.stderr,
      )?));
    }

    Ok(())
  }

  async fn push(&self, repository: &String) -> miette::Result<(), EcsHelperVarietyError> {
    let latest_tag: String = format!("{}:latest", repository);
    let version_tag: String = format!("{}:{}", repository, self.config.version);

    let mut push_latest_command = TokioCommand::new("docker");
    push_latest_command.arg("push").arg(&latest_tag);

    let mut push_version_command = TokioCommand::new("docker");
    push_version_command.arg("push").arg(&version_tag);

    log::info!("Pushing with two tags: {} & {}", latest_tag, version_tag);

    try_join!(
      async {
        let push_latest_output = push_latest_command.output().await?;
        if !push_latest_output.status.success() {
          return Err(EcsHelperVarietyError::PushImageError(format!(
            "Failed to push image with tag {}",
            latest_tag
          )));
        }
        Ok(())
      },
      async {
        let push_version_output = push_version_command.output().await?;
        if !push_version_output.status.success() {
          return Err(EcsHelperVarietyError::PushImageError(format!(
            "Failed to push image with tag {}",
            version_tag
          )));
        }
        Ok(())
      }
    )?;

    Ok(())
  }
}

impl Command for BuildAndPushCommand {
  fn name(&self) -> String {
    "build-and-push".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let Config {
      sdk_config,
      region,
      aws_account_id,
      ..
    } = &self.config;

    let repository = self.get_repository().await?;
    let auth_output = auth::login_to_ecr(sdk_config, region, aws_account_id).await?;

    if !auth_output.status.success() {
      return Err(EcsHelperVarietyError::LoginFailed(format!(
        "Login failed with status code: {}",
        auth_output.status.code().unwrap_or(0)
      )));
    }

    if self.should_cache {
      self.pull_image_to_cache(&repository).await?;
    }

    self.build(&repository).await?;
    self.push(&repository).await?;

    Ok(())
  }
}
