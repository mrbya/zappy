use std::path::{Path, PathBuf};
use std::{fs, panic};

use sha2::{Digest, Sha256};

fn main() {
    let templates_dir = PathBuf::from("templates");

    println!("cargo:rerun-if-changed=templates");

    let hash = hash_templates(&templates_dir)
        .unwrap_or_else(|error| panic!("failed to hash bundled templates: {error}"));

    println!("cargo:rustc-env=ZAPPY_TEMPLATES_HASH=sha256:{hash}");
}

fn hash_templates(path: &Path) -> std::io::Result<String> {
    let mut files = Vec::new();
    collect_files(path, &mut files)?;
    files.sort();

    let mut hasher = Sha256::new();

    for file in files {
        let relative = file.strip_prefix(path).unwrap_or(&file);

        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update([0]);

        let bytes = fs::read(&file)?;
        hasher.update(bytes);
        hasher.update([0]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }

    Ok(())
}
