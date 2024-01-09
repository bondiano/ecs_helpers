use clap::Parser;
use ecs_helpers::{
  args::{Args, Commands},
  config::Config,
};
use env_logger::Env;
use miette::Context;

use crate::commands::{ExportImagesCommandOptions, LoginCommandOptions, RunCommandCommand};

mod commands;

#[tokio::main]
async fn main() -> miette::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

  let args = Args::parse();
  let config = Config::new(&args).await?;

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
    Commands::ExportImages { application } => {
      let command_options = ExportImagesCommandOptions::new(config, application).await?;

      log::info!("Export images");

      let export_images_result = commands::export_images(command_options)
        .await
        .wrap_err("ecs_helpers::export_images");

      match export_images_result {
        Ok(result) => {
          log::info!("Export images was successful");

          println!("{result}");
          Ok(())
        }
        Err(error) => {
          log::error!("Export images was failed: {}", error);
          Err(error)
        }
      }
    }
    Commands::RunCommand {
      cluster,
      service,
      timeout,
      command,
      name,
      container,
    } => {
      log::info!("Run command: {}", command);

      let run_command_command =
        RunCommandCommand::new(config, timeout, command, cluster, service, name, container).await?;

      let run_command_result = run_command_command
        .run()
        .await
        .wrap_err("ecs_helpers::run_command");

      match run_command_result {
        Ok(_) => {
          log::info!("Run command was successful");
          Ok(())
        }
        Err(error) => {
          log::error!("Run command was failed: {}", error);
          Err(error)
        }
      }
    }
  }
}
