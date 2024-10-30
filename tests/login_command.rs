#[test]
fn test_ecr_login_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("ecr_login")
    .assert();
}
