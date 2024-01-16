#[test]
fn test_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_ecr_login_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("ecr_login")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_login_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("login")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_export_images_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("export_images")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_run_command_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("run_command")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_export_env_secrets_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("export_env_secrets")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_build_and_push_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("build_and_push")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_deploy_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("deploy")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}
