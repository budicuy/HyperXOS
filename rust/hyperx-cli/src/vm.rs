use std::fs;
use std::path::Path;
use std::process::Command;

use crate::util::{command_exists, find_latest_iso, run_stream};

pub fn run_vm(root: &Path, mode: &str) -> Result<(), String> {
    println!("==> [HyperXOS] Initializing QEMU virtual machine test harness (Mode: {})...", mode);

    let disk_image = root.join("hyperxos-install.qcow2");
    let disk_size = "40G";

    let ovmf_code = Path::new("/usr/share/edk2/x64/OVMF_CODE.4m.fd");
    let ovmf_vars_orig = Path::new("/usr/share/edk2/x64/OVMF_VARS.4m.fd");
    let ovmf_vars_local = root.join("OVMF_VARS.4m.fd");

    let latest_iso = find_latest_iso(root);

    // Ensure OVMF NVRAM copy exists
    if !ovmf_vars_local.exists() && ovmf_vars_orig.exists() {
        println!("  -> Creating local UEFI NVRAM copy ({})...", ovmf_vars_local.display());
        let _ = fs::copy(ovmf_vars_orig, &ovmf_vars_local);
    }

    // Build QEMU arguments
    let mut qemu = Command::new("qemu-system-x86_64");

    // KVM Acceleration
    let kvm = Path::new("/dev/kvm");
    if kvm.exists() {
        if let Ok(meta) = fs::metadata(kvm) {
            use std::os::unix::fs::PermissionsExt;
            let mode_val = meta.permissions().mode();
            if mode_val & 0o006 != 0 || mode_val & 0o060 != 0 {
                qemu.args(["-enable-kvm", "-cpu", "host"]);
            } else {
                qemu.args(["-cpu", "max"]);
            }
        } else {
            qemu.args(["-cpu", "max"]);
        }
    } else {
        qemu.args(["-cpu", "max"]);
    }

    // Resources & Basic Devices
    qemu.args([
        "-smp", "4",
        "-m", "4096",
        "-netdev", "user,id=net0",
        "-device", "virtio-net-pci,netdev=net0,romfile=",
        "-chardev", "socket,path=/tmp/qga.sock,server=on,wait=off,id=qga0",
        "-device", "virtio-serial",
        "-device", "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0",
        "-monitor", "unix:/tmp/hmp.sock,server=on,wait=off",
        "-qmp", "unix:/tmp/qmp.sock,server=on,wait=off",
    ]);

    // Display Backend (prefer SDL OpenGL virgl acceleration)
    let display_env = std::env::var("DISPLAY").or_else(|_| std::env::var("WAYLAND_DISPLAY"));
    if display_env.is_ok() {
        // Test SDL virgl
        let test_sdl = Command::new("qemu-system-x86_64")
            .args(["-display", "sdl,gl=on", "-device", "virtio-vga-gl", "-help"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if test_sdl {
            println!("  -> Using hardware-accelerated SDL OpenGL (virgl 3D) display backend.");
            qemu.args(["-device", "virtio-vga-gl", "-display", "sdl,gl=on"]);
        } else {
            println!("  -> Using standard GTK display backend.");
            qemu.args(["-device", "virtio-vga", "-display", "gtk"]);
        }
    } else {
        qemu.args(["-device", "virtio-vga", "-display", "none"]);
    }

    // Audio Device Setup
    if command_exists("pipewire") || std::env::var("PULSE_SERVER").is_ok() {
        qemu.args(["-audiodev", "pa,id=snd0", "-device", "intel-hda", "-device", "hda-duplex,audiodev=snd0"]);
    }

    // Attach UEFI firmware
    if ovmf_code.exists() && ovmf_vars_local.exists() {
        qemu.args([
            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code.display()),
            "-drive", &format!("if=pflash,format=raw,file={}", ovmf_vars_local.display()),
        ]);
    }

    match mode {
        "dev" | "live" => {
            let iso = latest_iso.ok_or_else(|| {
                "No HyperXOS ISO found in out/. Please run 'hyperx build' first.".to_string()
            })?;

            // Reset UEFI NVRAM for live dev mode so it cleanly boots the ISO
            if ovmf_vars_orig.exists() {
                let _ = fs::copy(ovmf_vars_orig, &ovmf_vars_local);
            }

            // Ensure virtual disk exists so installer can test partitioning
            if !disk_image.exists() {
                let mut img = Command::new("qemu-img");
                img.args(["create", "-f", "qcow2", disk_image.to_str().unwrap(), disk_size]);
                let _ = img.status();
            }

            println!("==> [HyperXOS] Booting Live ISO in dev mode: {}", iso.display());
            qemu.args([
                "-drive", &format!("file={},format=qcow2,if=virtio", disk_image.display()),
                "-drive", &format!("file={},media=cdrom,readonly=on", iso.display()),
                "-boot", "order=d,menu=on",
            ]);

            run_stream(&mut qemu, "qemu live session")?;
        }

        "install" | "vm-install" => {
            let iso = latest_iso.ok_or_else(|| {
                "No HyperXOS ISO found in out/. Please run 'hyperx build' first.".to_string()
            })?;

            println!("==> [HyperXOS] Resetting virtual disk {} to clean {} image...", disk_image.display(), disk_size);
            let _ = fs::remove_file(&disk_image);
            let mut img = Command::new("qemu-img");
            img.args(["create", "-f", "qcow2", disk_image.to_str().unwrap(), disk_size]);
            let _ = run_stream(&mut img, "qemu-img create");

            // Reset UEFI NVRAM for fresh install
            if ovmf_vars_orig.exists() {
                let _ = fs::copy(ovmf_vars_orig, &ovmf_vars_local);
            }

            println!("==> [HyperXOS] Booting installer environment with target disk attached...");
            qemu.args([
                "-drive", &format!("file={},format=qcow2,if=virtio", disk_image.display()),
                "-drive", &format!("file={},media=cdrom,readonly=on", iso.display()),
                "-boot", "order=d,menu=on",
            ]);

            run_stream(&mut qemu, "qemu install session")?;
        }

        "run" => {
            if !disk_image.exists() {
                return Err(format!(
                    "Virtual disk '{}' not found. Please run 'hyperx install' first.",
                    disk_image.display()
                ));
            }

            println!("==> [HyperXOS] Booting installed system from virtual disk: {}", disk_image.display());
            qemu.args([
                "-drive", &format!("file={},format=qcow2,if=virtio", disk_image.display()),
                "-boot", "order=c,menu=on",
            ]);

            run_stream(&mut qemu, "qemu run session")?;
        }

        other => {
            return Err(format!("Unknown VM mode '{}'. Valid modes: dev, install, run.", other));
        }
    }

    Ok(())
}
