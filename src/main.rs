use clap::{Parser, Subcommand};
use ecs_helpers::config::Config;
use env_logger::Env;
use miette::Context;

use crate::commands::{ExportImagesCommandOptions, LoginCommandOptions};

mod commands;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
  #[clap(short, long, env)]
  environment: Option<String>,

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

  // Prints images for the project and application
  #[clap(alias = "export_images")]
  ExportImages {
    // Project name
    #[clap(short, long, env)]
    project: String,

    // Application name
    #[clap(short, long, env)]
    application: String,
  },
}

#[tokio::main]
async fn main() -> miette::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

  let args = Args::parse();
  let config = Config::new(args.environment).await?;

  match args.cmd {
    Commands::Login { aws_account_id } => {
      let command_options = LoginCommandOptions::new(aws_account_id).await?;

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
    Commands::ExportImages {
      project,
      application,
    } => {
      let command_options: ExportImagesCommandOptions =
        ExportImagesCommandOptions::new(config, project, application).await?;

      log::info!("Export images");

      let export_images_result = commands::export_images(command_options)
        .await
        .wrap_err("ecs_helpers::export_images");

      match export_images_result {
        Ok(result) => {
          log::info!("Export images was successful");

          println!("{}", result);
          Ok(())
        }
        Err(error) => {
          log::error!("Export images was failed: {}", error);
          Err(error)
        }
      }
    }
  }
}
