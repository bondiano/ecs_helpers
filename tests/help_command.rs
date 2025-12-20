use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_ecr_login_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("ecr_login")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_login_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("login")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_export_images_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("export_images")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_run_command_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("run_command")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_export_env_secrets_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("export_env_secrets")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_build_and_push_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("build_and_push")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}

#[test]
fn test_deploy_help() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("deploy")
    .arg("--help")
    .assert()
    .success()
    .stderr("");
}
