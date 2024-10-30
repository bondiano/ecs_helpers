use miette::Context;

use crate::errors::EcsHelperVarietyError;

pub trait Command {
  fn run(
    &self,
  ) -> impl std::future::Future<Output = miette::Result<(), EcsHelperVarietyError>> + Send;
  fn name(&self) -> String;
}

pub async fn run_command<T: Command>(command: T) -> miette::Result<()> {
  let error_message = format!("ecs_helpers::{}", command.name());
  let command_result = command.run().await.wrap_err(error_message);

  command_result.map_err(|error| {
    log::error!("{} was filed with:\n{}", command.name(), error);
    error
  })
}
