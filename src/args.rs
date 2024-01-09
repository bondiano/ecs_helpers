use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
  #[clap(long, env, default_value = "false")]
  pub use_image_tag_env_prefix: bool,

  /// Set environment, it there is empty will try to detect based on the branch name
  #[clap(short, long, env)]
  pub environment: Option<String>,

  /// Set version which will be applied to all containers in the task if tag is present in the repo
  #[clap(short, long, env)]
  pub version: Option<String>,

  /// Set project name, will be used to detect cluster
  #[clap(short, long, env)]
  pub project: String,

  /// Set application name, will be used to detect service and task definition
  #[clap(short, long, env)]
  pub application: Option<String>,

  #[clap(subcommand)]
  pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Login to AWS ECR. It assumes that you have already set up your AWS credentials.
  #[clap(alias = "ecs_login")]
  Login {
    /// The AWS account ID
    #[clap(long, env)]
    aws_account_id: String,
  },

  /// Prints images for the project and application
  #[clap(alias = "export_images")]
  ExportImages {
    /// Set application name, will be used to detect service and task definition
    #[clap(short, long, env)]
    application: String,
  },

  /// Run command on ECS cluster
  #[clap(alias = "run_command")]
  RunCommand {
    /// Set command, should not demonize container
    #[clap(short, long, env)]
    command: String,

    /// Set timeout in seconds how long to wait until deployment finished
    #[clap(short, long, env, default_value = "600")]
    timeout: u64,

    /// Set cluster name, could be auto-detected if project and environment are specified
    #[clap(long, env)]
    cluster: Option<String>,

    /// Set service, could be auto-detected if application and environment are specified
    #[clap(short, long, env)]
    service: Option<String>,

    /// Set name (will be used for task definition name and log prefix)
    #[clap(short, long, env)]
    name: Option<String>,

    /// Set container name (default is the first container in the task definition)
    #[clap(long, env, alias = "container-name")]
    container: Option<String>,
  },
}
