use clap::{Parser, Subcommand};
use env_logger::Env;
use miette::Context;

use crate::commands::LoginCommandOptions;

mod commands;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
  #[clap(subcommand)]
  cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Login to AWS ECR. It assumes that you have already set up your AWS credentials.
  #[clap(alias = "ecs_login")]
  Login {
    /// The AWS account ID
    #[clap(long, env)]
    aws_account_id: String,
  },
}

#[tokio::main]
async fn main() -> miette::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

  let args = Args::parse();

  match args.cmd {
    Commands::Login { aws_account_id } => {
      let command_options: LoginCommandOptions = LoginCommandOptions::new(aws_account_id).await?;

      log::info!("Login to AWS ECR");

      let login_result = commands::login(command_options)
        .await
        .wrap_err("ecs_helpers::login");

      match login_result {
        Ok(_) => {
          log::info!("Login to AWS ECR was successful");
          Ok(())
        }
        Err(error) => {
          log::error!("Login to AWS ECR was failed: {}", error);
          Err(error)
        }
      }
    }
  }
}
