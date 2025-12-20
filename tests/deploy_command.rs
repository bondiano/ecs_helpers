use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_deploy_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--project")
    .arg("test")
    .arg("--environment")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("deploy")
    .arg("--timeout")
    .arg("1")
    .arg("--cluster")
    .arg("test")
    .arg("--service")
    .arg("test")
    .assert()
    .failure();
}
