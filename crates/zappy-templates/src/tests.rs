use std::fs;

use crate::cache::CACHE_MARKER;
use crate::{clear_cache_dir, ensure_bundled_templates_available};

#[test]
fn extracts_bundled_templates_and_clear_clears_cache() {
    let templates_dir =
        ensure_bundled_templates_available().expect("bundled templates should be extracted");

    assert!(templates_dir.exists());

    let expected_ids = [
        "cpp-cmake-app",
        "cpp-cmake-lib",
        "lua-cli",
        "nvim-plugin",
        "rust-cli",
    ];

    for id in expected_ids {
        let manifest_path = templates_dir.join(id).join("zappy.toml");
        let manifest = zappy_core::Manifest::load_from_path(&manifest_path)
            .expect("bundled manifest should parse");

        assert_eq!(manifest.template.id.as_str(), id);
    }

    assert!(templates_dir.join(CACHE_MARKER).exists());

    let reused_templates_dir =
        ensure_bundled_templates_available().expect("ready cache should be reused");

    assert_eq!(templates_dir, reused_templates_dir);

    fs::remove_file(templates_dir.join(CACHE_MARKER)).expect("file should delete");
    let new_templates_dir =
        ensure_bundled_templates_available().expect("cache re-build should pass");

    assert_eq!(templates_dir, new_templates_dir);
    assert!(templates_dir.join(CACHE_MARKER).exists());

    clear_cache_dir().expect("cache should clear");

    assert!(!templates_dir.exists());

    clear_cache_dir().expect("clearing missing cache should still succeed");
}
