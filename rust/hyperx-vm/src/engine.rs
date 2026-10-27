use std::path::Path;
use std::process::Command;

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
    pub fn run_install(root: &Path, config: &VmConfig) -> Result<(), String> {
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

        println!("==> [hyperx-vm] Starting virtual machine (Installer Mode)...");
        run_stream(&mut cmd, "hyperx-vm install")
    }

    /// Boot installed HyperXOS system directly from 40GB virtual disk
    pub fn run_installed(root: &Path, config: &VmConfig) -> Result<(), String> {
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
        run_stream(&mut cmd, "hyperx-vm run")
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

        // 7. Direct OpenGL Host GPU Display Acceleration
        let (_backend, display_args) = DisplayManager::detect(root);
        for arg in display_args {
            cmd.arg(arg);
        }

        Ok(cmd)
    }
}
