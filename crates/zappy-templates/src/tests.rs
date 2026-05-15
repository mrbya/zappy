use crate::{clear_cache_dir, ensure_bundled_templates_available};

#[test]
fn extracts_bundled_templates_and_clear_clears_cache() {
    let templates_dir =
        ensure_bundled_templates_available().expect("bundled templates should be extracted");

    assert!(templates_dir.exists());
    assert!(templates_dir.join("rust-cli/zappy.toml").exists());

    clear_cache_dir().expect("cache should clear");

    assert!(!templates_dir.exists());
}

#[test]
fn empty_cache_clear_passes() {
    clear_cache_dir().expect("empty cache should clear");
}
