use std::{
    env, fs,
    path::{Path, PathBuf},
};
fn visit(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) {
    println!("cargo:rerun-if-changed={}", dir.display());
    for entry in fs::read_dir(dir).expect("read packaged web directory") {
        let path = entry.unwrap().path();
        assert!(
            !path.is_symlink(),
            "packaged web files must not be symlinks"
        );
        if path.is_dir() {
            visit(root, &path, files)
        } else if path.is_file() {
            files.push(path.strip_prefix(root).unwrap().to_owned());
        }
    }
}
fn main() {
    println!("cargo:rerun-if-env-changed=TANGENT_WEB_DIST");
    let mut code = String::from("pub static ASSETS: &[(&str, &[u8])] = &[\n");
    if let Some(value) = env::var_os("TANGENT_WEB_DIST") {
        let root = PathBuf::from(value)
            .canonicalize()
            .expect("TANGENT_WEB_DIST must exist");
        for required in [
            "index.html",
            "pkg/tangent_player_bg.wasm",
            "samples/bach-prelude-bwv924a.mid",
            "samples/ATTRIBUTION.md",
        ] {
            assert!(
                root.join(required).is_file(),
                "missing required web asset: {required}"
            );
        }
        let mut files = Vec::new();
        visit(&root, &root, &mut files);
        files.sort();
        for file in files {
            let url = format!("/{}", file.to_string_lossy().replace('\\', "/"));
            code.push_str(&format!(
                "({url:?}, include_bytes!({:?})),\n",
                root.join(file)
            ));
        }
    }
    code.push_str("];\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("assets.rs"),
        code,
    )
    .unwrap();
}
