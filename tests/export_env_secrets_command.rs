use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_export_env_secrets_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("export_env_secrets")
    .assert()
    .failure();
}
