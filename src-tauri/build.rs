use std::process::Command;

fn main() {
    Command::new("sh")
        .arg("-c")
        .arg("cargo install warp-gui-app")
        .output()
        .expect("Build error");
    tauri_build::build()
}
