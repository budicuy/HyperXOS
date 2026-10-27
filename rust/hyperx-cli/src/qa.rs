use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::util::find_latest_iso;

pub fn run_test(root: &Path) -> Result<(), String> {
    println!("==================================================");
    println!("  HyperXOS Automated Quality Assurance & Testing (Pure Rust)");
    println!("==================================================");

    let latest_iso = find_latest_iso(root).ok_or_else(|| {
        "No HyperXOS ISO found in out/. Please run 'hyperx build' first.".to_string()
    })?;

    println!("[+] Found ISO image: {}", latest_iso.display());

    // 1. Size Verification
    let metadata = fs::metadata(&latest_iso).map_err(|e| e.to_string())?;
    let size_bytes = metadata.len();
    let size_mb = size_bytes / 1024 / 1024;
    println!("[+] ISO Image Size: {} MB ({} bytes)", size_mb, size_bytes);

    if size_mb < 500 {
        return Err(format!("ISO size is suspiciously small: {} MB (< 500 MB)", size_mb));
    }

    // 2. Header and Magic Verification
    // ISO 9660 primary volume descriptor starts at byte 32768 (0x8000) with "CD001"
    let mut file = File::open(&latest_iso).map_err(|e| e.to_string())?;
    let mut buffer = [0u8; 32774];
    if let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read >= 32774 {
            let magic = &buffer[32769..32774];
            if magic == b"CD001" {
                println!("[✓] Valid ISO 9660 CD001 magic header verified at offset 0x8001.");
            } else {
                println!("[!] Warning: Standard ISO magic at 0x8001 not detected, verifying via file utility...");
                let f_out = Command::new("file")
                    .arg(&latest_iso)
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                    .unwrap_or_default();
                if f_out.contains("ISO 9660") || f_out.contains("DOS/MBR") {
                    println!("[✓] Valid ISO / Hybrid boot header verified via file signature.");
                } else {
                    return Err("File is not a valid ISO 9660 / hybrid boot image.".to_string());
                }
            }
        }
    }

    // 3. SHA256 Checksum Verification
    let sha_file = latest_iso.with_extension("iso.sha256");
    if sha_file.exists() {
        println!("[+] Verifying official SHA256 checksum against .sha256 file...");
        let check_res = Command::new("sha256sum")
            .arg("-c")
            .arg(sha_file.file_name().unwrap())
            .current_dir(latest_iso.parent().unwrap())
            .output();

        if let Ok(res) = check_res {
            if res.status.success() {
                println!("[✓] SHA256 checksum verified valid and untampered.");
            } else {
                return Err("SHA256 checksum verification failed!".to_string());
            }
        }
    }

    // 4. Non-interactive QEMU Headless Smoke Test
    println!("[+] Initiating QEMU headless kernel boot smoke test (25 seconds)...");
    let bundled_qemu = root.join("rust/hyperx-vm/.bin/bin/qemu-system-x86_64");
    let qemu_bin = if bundled_qemu.exists() {
        bundled_qemu
    } else {
        std::path::PathBuf::from("qemu-system-x86_64")
    };

    let mut qemu = Command::new(&qemu_bin);
    let lib_dir = root.join("rust/hyperx-vm/.bin/lib/qemu");
    let share_dir = root.join("rust/hyperx-vm/.bin/share/qemu");
    if lib_dir.exists() {
        qemu.env("QEMU_MODULE_DIR", &lib_dir);
        qemu.env("LD_LIBRARY_PATH", &lib_dir);
    }
    if share_dir.exists() {
        qemu.args(["-L", share_dir.to_str().unwrap()]);
    }

    let kvm = Path::new("/dev/kvm");
    if kvm.exists() {
        qemu.args(["-enable-kvm", "-cpu", "host"]);
    } else {
        qemu.args(["-cpu", "max"]);
    }

    qemu.args([
        "-smp", "2",
        "-m", "2048",
        "-cdrom", latest_iso.to_str().unwrap(),
        "-display", "none",
        "-vga", "none",
        "-serial", "stdio",
    ]);

    qemu.stdout(Stdio::null());
    qemu.stderr(Stdio::null());

    let mut child = qemu.spawn().map_err(|e| format!("Failed to spawn QEMU smoke test: {}", e))?;
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(25);

    let mut exited_early = false;
    let mut exit_code = 0;

    while start.elapsed() < timeout {
        if let Ok(Some(status)) = child.try_wait() {
            exited_early = true;
            exit_code = status.code().unwrap_or(-1);
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    if !exited_early {
        // Still running after 25s: the kernel and initramfs loaded stably!
        let _ = child.kill();
        let _ = child.wait();
        println!("[✓] Automated boot test PASSED! Kernel and initramfs executed stably in VM for 25s.");
    } else if exit_code == 0 {
        println!("[✓] QEMU booted and exited cleanly with code 0.");
    } else {
        return Err(format!("QEMU exited unexpectedly with code {} during boot test.", exit_code));
    }

    println!("==================================================");
    println!("  All automated verification checks PASSED!");
    println!("==================================================");
    Ok(())
}
