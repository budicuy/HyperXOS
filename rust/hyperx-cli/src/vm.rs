use std::path::Path;
use std::process::Command;

use hyperx_ui_kit::run_stream;

pub fn run_vm(root: &Path, mode: &str, auto: bool) -> Result<(), String> {
    let hyperx_vm_bin = root.join("rust/target/release/hyperx-vm");

    // Only build if the binary is missing (the ./hyperx wrapper already keeps both hyperx and hyperx-vm up to date)
    if !hyperx_vm_bin.exists() {
        let mut build = Command::new("cargo");
        build.args([
            "build",
            "--release",
            "--manifest-path",
            root.join("rust/Cargo.toml").to_str().unwrap(),
            "--bin",
            "hyperx-vm",
        ]);
        run_stream(&mut build, "cargo build hyperx-vm")?;
    }

    println!("==> [HyperXOS] Launching Virtual Machine via hyperx-vm (Mode: {}, auto: {})...", mode, auto);
    let mut cmd = Command::new(&hyperx_vm_bin);
    if auto {
        cmd.arg("--auto");
    }
    cmd.arg(mode);
    run_stream(&mut cmd, "hyperx-vm")
}
