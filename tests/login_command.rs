use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_ecr_login_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("--aws-account-id")
    .arg("123456789012")
    .arg("ecr_login")
    .assert();
}
