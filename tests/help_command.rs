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
fn test_ecs_login_help() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("ecs_login")
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
