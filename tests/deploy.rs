#[test]
fn test_deploy_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--environment")
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
