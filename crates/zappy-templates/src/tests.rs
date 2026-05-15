use std::fs;

use crate::cache::CACHE_MARKER;
use crate::{clear_cache_dir, ensure_bundled_templates_available};

#[test]
fn extracts_bundled_templates_and_clear_clears_cache() {
    let templates_dir =
        ensure_bundled_templates_available().expect("bundled templates should be extracted");

    assert!(templates_dir.exists());
    assert!(templates_dir.join("rust-cli/zappy.toml").exists());
    assert!(templates_dir.join("lua-cli/zappy.toml").exists());
    assert!(templates_dir.join(CACHE_MARKER).exists());

    fs::remove_file(templates_dir.join(CACHE_MARKER)).expect("file should delete");
    let new_templates_dir =
        ensure_bundled_templates_available().expect("cache re-build should pass");

    assert_eq!(templates_dir, new_templates_dir);
    assert!(templates_dir.join(CACHE_MARKER).exists());

    clear_cache_dir().expect("cache should clear");

    assert!(!templates_dir.exists());
}
