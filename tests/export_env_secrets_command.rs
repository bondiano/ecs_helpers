#[test]
fn test_export_env_secrets_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("export_env_secrets")
    .assert()
    .failure();
}
