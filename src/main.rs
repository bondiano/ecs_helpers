use clap::Parser;
use ecs_helpers::{
  args::{CommandArguments, Commands},
  command::run_command,
  config::Config,
};
use env_logger::Env;

use crate::commands::{ExportImagesCommand, LoginCommand, RunCommandCommand};

mod commands;

#[tokio::main]
async fn main() -> miette::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let args = CommandArguments::parse();
  let config = Config::new(&args).await?;

  match args.cmd {
    Commands::Login(options) => {
      let login_command = LoginCommand::new(config, options);

      run_command(login_command).await
    }
    Commands::ExportImages(options) => {
      let export_images_command = ExportImagesCommand::new(config, options);

      run_command(export_images_command).await
    }
    Commands::RunCommand(options) => {
      log::info!("Run command: {}", options.command);

      let run_command_command = RunCommandCommand::new(config, options);

      run_command(run_command_command).await
    }
  }
}
