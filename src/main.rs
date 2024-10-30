use clap::Parser;
use commands::{BuildAndPushCommand, DeployCommand, ExportEnvSecretsCommand};
use ecs_helpers::{
  args::{CommandArguments, Commands},
  command::run_command,
  config::Config,
};

use crate::commands::{ExecCommand, ExportImagesCommand, LoginCommand, RunCommandCommand};

mod commands;

#[tokio::main]
async fn main() -> miette::Result<()> {
  tracing_subscriber::fmt::init();

  let args = CommandArguments::parse();
  log::debug!("Run with arguments: {:?}", args);

  let config = Config::new(&args).await?;
  log::debug!("Config: {:?}", config);

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
      let run_command_command = RunCommandCommand::new(config, options);
      run_command(run_command_command).await
    }
    Commands::Exec(options) => {
      let exec_command = ExecCommand::new(config, options);
      run_command(exec_command).await
    }
    Commands::ExportEnvSecrets(options) => {
      let export_env_secrets_command = ExportEnvSecretsCommand::new(config, options);
      run_command(export_env_secrets_command).await
    }
    Commands::BuildAndPush(options) => {
      let build_and_push_command = BuildAndPushCommand::new(config, options);
      run_command(build_and_push_command).await
    }
    Commands::Deploy(options) => {
      let deploy_command = DeployCommand::new(config, options);
      run_command(deploy_command).await
    }
  }
}
