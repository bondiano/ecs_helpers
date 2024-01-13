#[test]
fn test_ecs_login_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("ecs_login")
    .assert();
}
