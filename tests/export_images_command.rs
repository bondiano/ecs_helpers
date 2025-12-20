use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_export_images_command_without_environment() {
  cargo_bin_cmd!("ecs_helpers")
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("export_images")
    .assert()
    .failure();
}
