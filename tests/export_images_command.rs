#[test]
fn test_export_images_command_without_environment() {
  assert_cmd::Command::cargo_bin("ecs_helpers")
    .unwrap()
    .arg("--project")
    .arg("test")
    .arg("--application")
    .arg("test")
    .arg("export_images")
    .assert()
    .failure();
}
