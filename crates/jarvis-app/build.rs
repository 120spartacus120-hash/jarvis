fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rerun-if-changed=jarvis-app-manifest.rc");
        println!("cargo:rerun-if-changed=jarvis-app-manifest.xml");

        embed_resource::compile("jarvis-app-manifest.rc", embed_resource::NONE)
            .manifest_required()
            .expect("Failed to embed Windows manifest for jarvis-app");
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = std::path::Path::new(&manifest_dir).join("..\\..\\lib\\windows\\amd64");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
}
