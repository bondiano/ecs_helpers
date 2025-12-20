use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_run_command_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--environment")
    .arg("test")
    .arg("run_command")
    .arg("-c")
    .arg("ls")
    .arg("--timeout")
    .arg("1")
    .arg("--cluster")
    .arg("test")
    .arg("--service")
    .arg("test")
    .arg("--name")
    .arg("test")
    .arg("--container")
    .arg("test")
    .assert()
    .failure();
}
