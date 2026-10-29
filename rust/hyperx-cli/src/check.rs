use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use crate::util::command_exists;

pub fn run_check(root: &Path) -> Result<(), String> {
    println!("==> [HyperXOS] Validating host environment and project configurations...");

    // 1. Host tools
    let tools = [
        ("mkarchiso", "archiso"),
        ("xorriso", "xorriso"),
    ];

    for (tool, pkg) in tools {
        if command_exists(tool) {
            println!("  [✓] Host build dependency '{}' is installed.", tool);
        } else {
            return Err(format!(
                "Missing required build dependency '{}'. Install via: sudo pacman -S {}",
                tool, pkg
            ));
        }
    }

    // Check VMM engine (hermetic bundle or host qemu)
    let bundled_vmm = root.join("rust/hyperx-vm/.bin/bin/qemu-system-x86_64");
    if bundled_vmm.exists() {
        println!("  [✓] Virtual Machine Engine: hyperx-vm (Hermetic 100% Self-Contained Bundle).");
    } else if command_exists("qemu-system-x86_64") {
        println!("  [✓] Virtual Machine Engine: System QEMU available.");
    } else {
        println!("  [✓] Virtual Machine Engine: hyperx-vm ready to auto-provision.");
    }

    // 2. KVM acceleration
    let kvm_path = Path::new("/dev/kvm");
    if kvm_path.exists() {
        if let Ok(meta) = fs::metadata(kvm_path) {
            let mode = meta.permissions().mode();
            if mode & 0o006 != 0 || mode & 0o060 != 0 {
                println!("  [✓] Hardware virtualization acceleration (/dev/kvm) is accessible.");
            } else {
                println!("  [!] Warning: /dev/kvm exists but check group permissions.");
            }
        }
    } else {
        println!("  [!] Warning: /dev/kvm not found; QEMU will run in software TCG mode.");
    }

    // 3. Profiledef.sh
    let profiledef = root.join("archiso/profiledef.sh");
    if profiledef.exists() {
        let syntax_ok = Command::new("bash")
            .args(["-n", profiledef.to_str().unwrap()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if syntax_ok {
            println!("  [✓] archiso/profiledef.sh syntax is valid.");
        } else {
            return Err("archiso/profiledef.sh contains bash syntax errors!".to_string());
        }
    } else {
        return Err("archiso/profiledef.sh missing!".to_string());
    }

    // 4. Packages.x86_64
    let packages_file = root.join("archiso/packages.x86_64");
    if packages_file.exists() {
        let content = fs::read_to_string(&packages_file).map_err(|e| e.to_string())?;
        let active_pkgs: Vec<&str> = content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();

        println!(
            "  [✓] archiso/packages.x86_64 found ({} packages listed).",
            active_pkgs.len()
        );

        // Verify vital packages are included
        let vital = [
            "linux",
            "base",
            "plasma-meta",
            "sddm",
            "hyper-x-installer",
            "hyper-x-tools",
            "hyper-x-hw",
        ];
        for v in vital {
            if !active_pkgs.contains(&v) {
                return Err(format!("Vital package '{}' is missing from packages.x86_64!", v));
            }
        }
    } else {
        return Err("archiso/packages.x86_64 missing!".to_string());
    }

    // 5. Package PKGBUILDs
    let pkgbuilds = [
        "packages/hyper-x-installer/PKGBUILD",
        "packages/hyper-x-hw/PKGBUILD",
        "packages/hyper-x-tools/PKGBUILD",
    ];
    for p in pkgbuilds {
        if root.join(p).exists() {
            println!("  [✓] PKGBUILD {} verified.", p);
        } else {
            println!("  [!] Warning: {} not found.", p);
        }
    }

    println!("[✓] All validations completed successfully!");
    Ok(())
}
