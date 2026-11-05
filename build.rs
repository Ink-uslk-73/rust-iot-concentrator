use std::path::Path;

fn main() {
    let dist = Path::new("ui/dist");
    if !dist.exists() {
        std::fs::create_dir_all(dist).expect("create ui/dist");
    }
    println!("cargo:rerun-if-changed=ui/dist");
}
