#[test]
fn test_build_and_push_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("build_and_push")
    .arg("--image")
    .arg("test")
    .assert();
}
