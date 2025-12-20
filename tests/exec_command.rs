use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_exec_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--environment")
    .arg("test")
    .arg("exec")
    .arg("-c")
    .arg("/bin/bash")
    .arg("--cluster")
    .arg("test")
    .arg("--service")
    .arg("test")
    .arg("--container")
    .arg("test")
    .assert()
    .failure();
}
