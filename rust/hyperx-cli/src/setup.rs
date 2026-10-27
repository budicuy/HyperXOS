use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use crate::util::{command_exists, run_stream};

pub fn run_setup(root: &Path) -> Result<(), String> {
    println!("======================================================================");
    println!("  HyperXOS Host Environment Setup (Pure Rust)");
    println!("======================================================================");

    println!("==> [1/3] Checking host system dependencies...");
    let dependencies = [
        ("mkarchiso", "archiso"),
        ("qemu-system-x86_64", "qemu-desktop"),
        ("xorriso", "xorriso"),
    ];

    let mut missing = Vec::new();
    for (bin, pkg) in dependencies {
        if command_exists(bin) {
            println!("  [✓] Required dependency '{}' is installed.", bin);
        } else {
            println!("  [-] Missing required dependency '{}'.", bin);
            missing.push(pkg);
        }
    }

    if !missing.is_empty() {
        return Err(format!(
            "Please install missing host packages via:\n    sudo pacman -S {}",
            missing.join(" ")
        ));
    }

    println!("==> [2/3] Checking KVM hardware acceleration...");
    let kvm = Path::new("/dev/kvm");
    if kvm.exists() {
        if let Ok(meta) = fs::metadata(kvm) {
            let mode = meta.permissions().mode();
            if mode & 0o006 != 0 || mode & 0o060 != 0 {
                println!("  [✓] Hardware acceleration /dev/kvm is accessible and ready.");
            } else {
                println!("  [!] Warning: /dev/kvm exists but current user may lack rw permissions.");
            }
        }
    } else {
        println!("  [!] Warning: /dev/kvm not found. QEMU will run in software TCG mode.");
    }

    println!("==> [3/3] Initializing project workspace directories and cache...");
    // 1. Work dir in /var/tmp
    let var_work = Path::new("/var/tmp/hyperxos-work");
    if !var_work.exists() {
        let mut install_cmd = Command::new("sudo");
        install_cmd.args(["install", "-d", "-m", "0777", "/var/tmp/hyperxos-work"]);
        let _ = run_stream(&mut install_cmd, "create /var/tmp/hyperxos-work");
    } else {
        let _ = fs::set_permissions(var_work, fs::Permissions::from_mode(0o777));
    }

    // 2. Local workspace directories
    let dirs = [
        root.join("out"),
        root.join("repo/x86_64"),
        root.join("work"),
    ];

    for d in dirs {
        fs::create_dir_all(&d).map_err(|e| format!("Failed to create {}: {}", d.display(), e))?;
        println!("  [✓] Directory ready: {}", d.display());
    }

    println!("[✓] HyperXOS host development environment successfully configured!");
    Ok(())
}
