use crate::ensure_bundled_templates_available;

#[test]
fn extracts_bundled_templates() {
    let templates_dir =
        ensure_bundled_templates_available().expect("bundled templates should be extracted");

    assert!(templates_dir.join("rust-cli/zappy.toml").exists());
}
