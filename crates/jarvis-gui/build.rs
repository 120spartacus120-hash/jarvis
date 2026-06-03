use std::path::{Path, PathBuf};

fn rerun_if_changed(path: &Path) {
    if path.exists() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn watch_frontend_dist(dist: &Path) {
    rerun_if_changed(&dist.join("index.html"));

    let assets = dist.join("assets");
    if let Ok(entries) = std::fs::read_dir(assets) {
        for entry in entries.flatten() {
            rerun_if_changed(&entry.path());
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let dist = manifest_dir.join("../../frontend/dist/client");
    watch_frontend_dist(&dist);

    let lib_path = manifest_dir.join("../../lib/windows/amd64");
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    tauri_build::build();
}
