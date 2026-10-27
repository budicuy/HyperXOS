use std::path::Path;
use std::process::Command;

use crate::util::run_stream;

pub fn run_vm(root: &Path, mode: &str) -> Result<(), String> {
    let hyperx_vm_bin = root.join("rust/hyperx-vm/target/release/hyperx-vm");

    if !hyperx_vm_bin.exists() {
        println!("==> [HyperXOS] Compiling 'hyperx-vm' (Pure Rust Virtual Machine Manager)...");
        let mut build = Command::new("cargo");
        build.args([
            "build",
            "--release",
            "--manifest-path",
            root.join("rust/hyperx-vm/Cargo.toml").to_str().unwrap(),
        ]);
        run_stream(&mut build, "cargo build hyperx-vm")?;
    }

    println!("==> [HyperXOS] Launching Virtual Machine via hyperx-vm (Mode: {})...", mode);
    let mut cmd = Command::new(&hyperx_vm_bin);
    cmd.arg(mode);
    run_stream(&mut cmd, "hyperx-vm")
}
