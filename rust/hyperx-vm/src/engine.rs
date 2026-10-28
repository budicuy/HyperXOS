use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::qga::{QgaClient, QGA_SOCKET};

use crate::config::VmConfig;
use crate::display::DisplayManager;
use crate::provisioner::VmmProvisioner;
use crate::storage::StorageManager;
use crate::util::run_stream;

pub struct VmEngine;

impl VmEngine {
    /// Boot HyperXOS Live ISO in hardware-accelerated VM
    pub fn run_live(root: &Path, config: &VmConfig) -> Result<(), String> {
        let iso = StorageManager::find_latest_iso(root).ok_or_else(|| {
            "No HyperXOS ISO found in out/. Please build one first using './hyperx build'.".to_string()
        })?;

        println!("======================================================================");
        println!("  HyperXOS Virtual Machine Engine — Live Environment");
        println!("  Firmware:   UEFI (OVMF 4M) + Limine Bootloader");
        println!("  ISO Image:  {}", iso.display());
        println!("  CPU / RAM:  {} Cores / {} MB", config.smp_cores, config.ram_mb);
        println!("======================================================================");

        // Ensure local UEFI NVRAM exists and reset so it cleanly boots the ISO
        StorageManager::ensure_nvram(config);
        StorageManager::reset_nvram(config);
        StorageManager::ensure_disk_exists(config)?;

        let mut cmd = Self::build_base_command(root, config)?;

        // Attach ISO and Disk
        cmd.args([
            "-cdrom", iso.to_str().unwrap(),
            "-drive", &format!("file={},format=qcow2,if=virtio", config.disk_image.display()),
            "-boot", "d",
        ]);

        println!("==> [hyperx-vm] Starting virtual machine (Live ISO Mode)...");
        run_stream(&mut cmd, "hyperx-vm live")
    }

    /// Run installation test on clean 40GB virtual disk
    pub fn run_install(root: &Path, config: &VmConfig, auto: bool) -> Result<(), String> {
        let iso = StorageManager::find_latest_iso(root).ok_or_else(|| {
            "No HyperXOS ISO found in out/. Please build one first using './hyperx build'.".to_string()
        })?;

        println!("======================================================================");
        println!("  HyperXOS Virtual Machine Engine — Installation Mode");
        println!("  Target:     40GB Clean Virtual Disk ({})", config.disk_image.display());
        println!("  Firmware:   UEFI (OVMF 4M) + Limine Bootloader");
        println!("  ISO Image:  {}", iso.display());
        println!("======================================================================");

        // Reset disk and NVRAM for clean fresh install
        StorageManager::reset_disk(config)?;
        StorageManager::ensure_nvram(config);
        StorageManager::reset_nvram(config);

        let mut cmd = Self::build_base_command(root, config)?;

        cmd.args([
            "-cdrom", iso.to_str().unwrap(),
            "-drive", &format!("file={},format=qcow2,if=virtio", config.disk_image.display()),
            "-boot", "d",
        ]);

        if !auto {
            println!("==> [hyperx-vm] Starting virtual machine (Manual Installer Mode)...");
            return run_stream(&mut cmd, "hyperx-vm install");
        }

        println!("==> [hyperx-vm] Starting virtual machine (Automated Install Test)...");
        let _ = std::fs::remove_file(QGA_SOCKET);
        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to start VM: {}", e))?;

        let result = Self::drive_auto_install();
        if result.is_err() {
            let _ = child.kill();
        }
        let _ = child.wait();
        result?;

        println!("==> [hyperx-vm] Automated install PASSED. Booting installed system...");
        Self::run_installed(root, config, true)
    }

    /// Drive `hyper-x-installer --auto-install /dev/vda` inside the live ISO via QEMU Guest Agent.
    fn drive_auto_install() -> Result<(), String> {
        println!("  -> Waiting for Live ISO guest agent (max 300s)...");
        let mut qga = QgaClient::wait_ready(Duration::from_secs(300))?;
        println!("  [✓] Guest agent connected.");

        // Let systemd finish starting core services (pacman-init, network)
        let _ = qga.exec("systemctl is-system-running --wait >/dev/null 2>&1; true");

        println!("  -> Launching: hyper-x-installer --auto-install /dev/vda");
        qga.spawn(
            "rm -f /tmp/autoinstall.log /tmp/autoinstall.exit; \
             /usr/bin/hyper-x-installer --auto-install /dev/vda > /tmp/autoinstall.log 2>&1; \
             echo $? > /tmp/autoinstall.exit",
        )?;

        let start = Instant::now();
        let mut offset: usize = 0;
        loop {
            std::thread::sleep(Duration::from_secs(3));
            // Stream new log bytes
            if let Ok((_, chunk)) =
                qga.exec(&format!("tail -c +{} /tmp/autoinstall.log 2>/dev/null", offset + 1))
            {
                if !chunk.is_empty() {
                    offset += chunk.len();
                    for line in chunk.lines() {
                        let l = line.trim();
                        if l.starts_with("[STAGE") || l.starts_with("[✓]") || l.starts_with("[-]") {
                            println!("  {}", l);
                        }
                    }
                }
            }
            if let Ok((0, code)) = qga.exec("cat /tmp/autoinstall.exit 2>/dev/null") {
                let code = code.trim();
                if !code.is_empty() {
                    if code == "0" {
                        println!(
                            "  [✓] Installation finished in {}s. Powering off live VM...",
                            start.elapsed().as_secs()
                        );
                        let _ = qga.spawn("sleep 1; systemctl poweroff");
                        return Ok(());
                    }
                    let (_, tail) = qga
                        .exec("tail -n 25 /tmp/autoinstall.log")
                        .unwrap_or_default();
                    return Err(format!(
                        "Automated install failed (exit {}). Last log:\n{}",
                        code, tail
                    ));
                }
            }
            if start.elapsed() > Duration::from_secs(2400) {
                return Err("Automated install timed out after 40 minutes".to_string());
            }
        }
    }

    /// Smoke-test the booted installed system: hyperx-de session alive + screenshot.
    fn smoke_test_installed(root: &Path) -> Result<(), String> {
        println!("  -> [smoke] Waiting for installed system guest agent (max 240s)...");
        let mut qga = QgaClient::wait_ready(Duration::from_secs(240))?;

        // 1. Sync latest hyperx-de package into guest if built in local repo
        let pkg_path = root.join("repo/x86_64/hyperx-de-1.0.0-1-x86_64.pkg.tar.zst");
        if pkg_path.exists() {
            println!("  -> [smoke] Syncing latest hyperx-de package into guest...");
            let server = Command::new("python3")
                .args(["-m", "http.server", "8099", "--directory", root.join("repo/x86_64").to_str().unwrap()])
                .spawn()
                .ok();
            std::thread::sleep(Duration::from_millis(600));

            let update_cmd = "curl -sSf http://10.0.2.2:8099/hyperx-de-1.0.0-1-x86_64.pkg.tar.zst -o /tmp/hyperx-de.pkg.tar.zst && \
                              pacman -U --noconfirm /tmp/hyperx-de.pkg.tar.zst && \
                              systemctl restart sddm";
            let (code, out) = qga.exec(update_cmd).unwrap_or((-1, String::new()));
            if let Some(mut s) = server {
                let _ = s.kill();
            }
            if code == 0 {
                println!("  [✓] [smoke] Latest hyperx-de package synced and installed in guest.");
            } else {
                eprintln!("  [!] [smoke] Guest package sync returned {}: {}", code, out);
            }
        }

        let start = Instant::now();
        loop {
            let (code, _) = qga.exec("pgrep -f 'hyperx-de --session-ui' >/dev/null")?;
            if code == 0 {
                break;
            }
            if start.elapsed() > Duration::from_secs(90) {
                let (_, log) = qga
                    .exec("journalctl -b -u sddm -n 30 --no-pager")
                    .unwrap_or_default();
                return Err(format!("hyperx-de session did not start. SDDM log:\n{}", log));
            }
            std::thread::sleep(Duration::from_secs(2));
        }
        println!("  [✓] [smoke] hyperx-de Wayland session is running.");

        let take_screenshot = |qga: &mut QgaClient, out_path: &Path| -> Result<(), String> {
            let (_, b64) = qga.exec(
                "WAYLAND_DISPLAY=$(ls /run/user/1000 | grep -E '^wayland-[0-9]+$' | head -n1) \
                 XDG_RUNTIME_DIR=/run/user/1000 grim /tmp/shot.png && base64 -w0 /tmp/shot.png",
            )?;
            let png = crate::qga::base64_decode(b64.trim());
            if png.len() > 1000 {
                let _ = std::fs::write(out_path, &png);
                println!("  [✓] [smoke] Screenshot saved: {}", out_path.display());
            }
            Ok(())
        };

        let send_hmp = |cmd: &str| {
            if let Ok(mut stream) = std::os::unix::net::UnixStream::connect("/tmp/hmp.sock") {
                let _ = std::io::Write::write_all(&mut stream, format!("{}\n", cmd).as_bytes());
            }
        };

        // 1. Pristine Desktop Screenshot
        std::thread::sleep(Duration::from_secs(3));
        let _ = take_screenshot(&mut qga, &root.join("out/vm-smoke-installed.png"));

        // 2. Open Start Menu via Ctrl+Space
        send_hmp("sendkey ctrl-spc");
        std::thread::sleep(Duration::from_secs(2));
        let _ = take_screenshot(&mut qga, &root.join("out/screenshot_hyperx_fluent_start_menu.png"));

        // 3. Close Start Menu and Open File Explorer via Super+E
        send_hmp("sendkey ctrl-spc");
        std::thread::sleep(Duration::from_millis(500));
        send_hmp("sendkey meta_l-e");
        std::thread::sleep(Duration::from_secs(2));
        let _ = take_screenshot(&mut qga, &root.join("out/screenshot_hyperx_fluent_explorer.png"));

        // 4. Close Explorer, Open Action Center / Quick Settings via Super+A
        send_hmp("sendkey alt-f4");
        std::thread::sleep(Duration::from_millis(500));
        send_hmp("sendkey meta_l-a");
        std::thread::sleep(Duration::from_secs(2));
        let _ = take_screenshot(&mut qga, &root.join("out/screenshot_hyperx_fluent_control_center.png"));

        println!("  [✓] [smoke] Installed system smoke test PASSED.");
        Ok(())
    }

    /// Boot installed HyperXOS system directly from 40GB virtual disk
    pub fn run_installed(root: &Path, config: &VmConfig, smoke: bool) -> Result<(), String> {
        if !config.disk_image.exists() {
            return Err(format!(
                "Virtual disk '{}' does not exist. Please run './hyperx install' first to install HyperXOS.",
                config.disk_image.display()
            ));
        }

        println!("======================================================================");
        println!("  HyperXOS Virtual Machine Engine — Installed System Mode");
        println!("  Boot Disk:  {}", config.disk_image.display());
        println!("  Firmware:   UEFI (OVMF 4M) + Installed Limine Bootloader");
        println!("  CPU / RAM:  {} Cores / {} MB", config.smp_cores, config.ram_mb);
        println!("======================================================================");

        StorageManager::ensure_nvram(config);

        let mut cmd = Self::build_base_command(root, config)?;

        cmd.args([
            "-drive", &format!("file={},format=qcow2,if=virtio", config.disk_image.display()),
            "-boot", "c",
        ]);

        println!("==> [hyperx-vm] Booting installed HyperXOS from virtual disk...");
        if !smoke {
            return run_stream(&mut cmd, "hyperx-vm run");
        }

        let _ = std::fs::remove_file(QGA_SOCKET);
        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to start VM: {}", e))?;

        let res = Self::smoke_test_installed(root);
        let _ = child.kill();
        let _ = child.wait();
        res
    }

    /// Build base QEMU/KVM command with UEFI firmware and hardware acceleration
    fn build_base_command(root: &Path, config: &VmConfig) -> Result<Command, String> {
        let ovmf_code = config.ovmf_code.as_ref().ok_or_else(|| {
            "OVMF UEFI firmware (OVMF_CODE.4m.fd) not found on host. Please install 'edk2-ovmf'.".to_string()
        })?;

        // 1. Ensure Hermetic Pinned VMM Binary
        let vmm_bin = VmmProvisioner::ensure_vmm(root)?;
        let mut cmd = Command::new(&vmm_bin);

        // 1b. Inject hermetic library and data directories for 100% self-contained isolation
        let bundle_dir = root.join("rust/hyperx-vm/.bin");
        let lib_dir = bundle_dir.join("lib/qemu");
        let share_dir = bundle_dir.join("share/qemu");

        if lib_dir.exists() {
            cmd.env("QEMU_MODULE_DIR", &lib_dir);
            cmd.env("LD_LIBRARY_PATH", &lib_dir);
        }

        if share_dir.exists() {
            cmd.args(["-L", share_dir.to_str().unwrap()]);
        }

        // 2. CPU & Virtualization Acceleration
        if config.has_kvm {
            println!("  [✓] Hardware Virtualization Acceleration: /dev/kvm (Host CPU Passthrough)");
            cmd.args(["-enable-kvm", "-cpu", "host"]);
        } else {
            println!("  [!] KVM not accessible. Falling back to software CPU virtualization.");
            cmd.args(["-cpu", "max"]);
        }

        // 3. Hardware resources
        cmd.args([
            "-smp", &config.smp_cores.to_string(),
            "-m", &config.ram_mb.to_string(),
        ]);

        // 4. UEFI Firmware
        cmd.args([
            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code.display()),
            "-drive", &format!("if=pflash,format=raw,file={}", config.ovmf_vars_local.display()),
        ]);

        // 5. Networking & Guest Services
        cmd.args([
            "-netdev", "user,id=net0",
            "-device", "virtio-net-pci,netdev=net0,romfile=",
            "-chardev", "socket,path=/tmp/qga.sock,server=on,wait=off,id=qga0",
            "-device", "virtio-serial",
            "-device", "virtserialport,chardev=qga0,name=org.qemu.guest_agent.0",
            "-monitor", "unix:/tmp/hmp.sock,server=on,wait=off",
            "-qmp", "unix:/tmp/qmp.sock,server=on,wait=off",
        ]);

        // 6. Audio backend
        cmd.args([
            "-audiodev", "pa,id=snd0",
            "-device", "intel-hda",
            "-device", "hda-duplex,audiodev=snd0",
        ]);

        // 6b. Absolute pointer (USB tablet on xHCI) so the guest cursor tracks the host
        //     mouse 1:1 without offset/grab issues in the QEMU window.
        cmd.args(["-device", "qemu-xhci,id=xhci", "-device", "usb-tablet,bus=xhci.0"]);

        // 7. Direct OpenGL Host GPU Display Acceleration
        let (_backend, display_args) = DisplayManager::detect(root);
        for arg in display_args {
            cmd.arg(arg);
        }

        Ok(cmd)
    }
}
