use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_build_and_push_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
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
  cargo_bin_cmd!("ecs_helpers")
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
  cargo_bin_cmd!("ecs_helpers")
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
