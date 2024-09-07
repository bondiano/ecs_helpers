use ecs_helpers::{
  args::ExecCommandArguments, cluster_helpers, config::Config, ecs::EcsClient,
  errors::EcsHelperVarietyError, service_helpers, ssm::SSMClient, task_helpers, Command,
};

use serde::{Deserialize, Serialize};
use tokio::process::Command as TokioCommand;

pub struct ExecCommand {
  ecs_client: EcsClient,
  ssm_client: SSMClient,
  config: Config,
  command: String,
  cluster: Option<String>,
  service: Option<String>,
  task: Option<String>,
  container: Option<String>,
}

impl ExecCommand {
  pub fn new(config: Config, args: ExecCommandArguments) -> Self {
    let sdk_config = &config.sdk_config;
    let ecs_client = EcsClient::new(sdk_config);
    let ssm_client = SSMClient::new(sdk_config);

    Self {
      ecs_client,
      ssm_client,
      config,
      cluster: args.cluster,
      service: args.service,
      task: args.task,
      command: args.command,
      container: args.container,
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SessionManagerJson {
  session_id: String,
  token_value: String,
  stream_url: String,
}

impl Command for ExecCommand {
  fn name(&self) -> String {
    "exec".to_string()
  }

  async fn run(&self) -> miette::Result<(), EcsHelperVarietyError> {
    let Config { region, .. } = &self.config;

    let cluster =
      cluster_helpers::get_current_cluster(&self.ecs_client, &self.config, &self.cluster).await?;

    let service =
      service_helpers::get_current_service(&self.ecs_client, &self.config, &cluster, &self.service)
        .await?;

    let task = task_helpers::get_current_task(
      &self.ecs_client,
      &self.config,
      &cluster,
      &service,
      &self.task,
    )
    .await?;

    let container = task_helpers::get_target_container(
      &self.ecs_client,
      &self.config,
      &cluster,
      &task,
      &self.container,
    )
    .await?;

    let session = self
      .ecs_client
      .execute_command(&cluster, &task, &container, &self.command)
      .await?;

    let session_id = session.clone().session_id.unwrap();

    let response = SessionManagerJson {
      session_id: session_id.clone(),
      token_value: session.token_value().unwrap().to_string(),
      stream_url: session.stream_url().unwrap().to_string(),
    };

    let response_string = serde_json::to_string(&response)?.clone();
    let mut session_manager_plugin = TokioCommand::new("session-manager-plugin");

    let run_command_output = session_manager_plugin
      .args([
        response_string.to_owned(),
        region.to_string(),
        "StartSession".to_string(),
        format!("https://ssm.{}.amazonaws.com/", region),
      ])
      .spawn()?;

    let _result = run_command_output.wait_with_output().await?;

    self.ssm_client.terminate_session(session_id).await?;

    Ok(())
  }
}
