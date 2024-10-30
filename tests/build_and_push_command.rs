#[test]
fn test_build_and_push_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("build_and_push")
    .arg("--image")
    .arg("test")
    .assert();
}

#[test]
fn test_build_and_push_command_with_custom_docker_file() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("build_and_push")
    .arg("--image")
    .arg("test")
    .arg("--directory")
    .arg("./frontend")
    .arg("--file")
    .arg("./frontend/apps/test/Dockerfile")
    .assert();
}

#[test]
fn test_build_and_push_command_with_target() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("build_and_push")
    .arg("--image")
    .arg("test")
    .arg("--target")
    .arg("test")
    .assert();
}
