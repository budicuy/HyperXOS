use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub const BASE_ONLINE_PACKAGES: &[&str] = &[
    // 1. Base Core & Essential System Utilities
    "base",
    "linux",
    "linux-firmware",
    "mkinitcpio",
    "which",
    "nano",
    "sudo",
    "bash-completion",
    "pciutils",
    "usbutils",
    "qemu-guest-agent",
    // 2. Filesystems & Storage Support
    "btrfs-progs",
    "e2fsprogs",
    "dosfstools",
    "cryptsetup",
    "gptfdisk",
    // 3. Networking Core
    "networkmanager",
    "wireless-regdb",
    "curl",
    // 4. Audio & Display Server Core
    "pipewire",
    "pipewire-pulse",
    "pipewire-alsa",
    "wireplumber",
    "mesa",
    "qt6-wayland",
    "xorg-xwayland",
    // 5. Bootloader (Default Limine)
    "limine",
    "efibootmgr",
    // 6. HyperXOS Branding, Keyrings & Management Tool
    "archlinux-keyring",
    "cachyos-keyring",
    "hyperxos-branding",
    "hyperxos-default-settings",
    "hyperxos-hooks",
    "hyperxos-keyring",
    "hyperxos-mirrorlist",
    "paru",
    "hyper-x-tools",
    // 7. Minimal Core KDE Plasma 6 Desktop
    "plasma-desktop",
    "plasma-workspace",
    "kwin",
    "sddm",
    "sddm-kcm",
    "dolphin",
    "konsole",
    "firefox",
    "plasma-nm",
    "plasma-pa",
    "powerdevil",
    "kscreen",
    "breeze",
    "breeze-gtk",
    "kde-gtk-config",
    "polkit-kde-agent",
    "xdg-user-dirs",
    "kpipewire",
    "kdialog",
];

pub struct PacstrapInstaller;

impl PacstrapInstaller {
    /// Initialize pacman configuration, local repository sync, and trust keys on host and target
    pub fn prepare_pacman_environment<F>(mut progress_callback: F) -> Result<(), String>
    where
        F: FnMut(&str),
    {
        progress_callback("Creating pacman directories on target /mnt...");
        let dirs = [
            "/mnt/etc/pacman.d",
            "/mnt/etc",
            "/mnt/usr/local/bin",
            "/mnt/opt/hyperxos-repo/x86_64",
            "/mnt/var/lib/pacman/sync",
        ];
        for d in dirs {
            let _ = fs::create_dir_all(d);
        }

        progress_callback("Copying network DNS and pacman configurations to /mnt...");
        let _ = fs::copy("/etc/resolv.conf", "/mnt/etc/resolv.conf");
        let _ = fs::copy("/etc/pacman.conf", "/mnt/etc/pacman.conf");

        // Copy /etc/pacman.d/*
        if let Ok(entries) = fs::read_dir("/etc/pacman.d") {
            for entry in entries.flatten() {
                let dest = Path::new("/mnt/etc/pacman.d").join(entry.file_name());
                if entry.path().is_file() {
                    let _ = fs::copy(entry.path(), dest);
                }
            }
        }

        // Copy local repo if present
        if Path::new("/opt/hyperxos-repo/x86_64").exists() {
            let _ = Command::new("cp")
                .args(["-af", "/opt/hyperxos-repo/x86_64/.", "/mnt/opt/hyperxos-repo/x86_64/"])
                .status();
        }

        // Copy pacman cache db sync if present
        if Path::new("/var/lib/pacman/sync").exists() {
            let _ = Command::new("cp")
                .args(["-rf", "/var/lib/pacman/sync/.", "/mnt/var/lib/pacman/sync/"])
                .status();
        }

        // Copy helper scripts
        for script in ["/usr/local/bin/hw-detect-gpu", "/usr/local/bin/hyperxos-postinstall"] {
            if Path::new(script).exists() {
                let dest = format!("/mnt{}", script);
                let _ = fs::copy(script, &dest);
                let _ = Command::new("chmod").args(["755", &dest]).status();
            }
        }

        // Initialize and populate pacman keys
        progress_callback("Populating pacman keyring (archlinux, cachyos, hyperxos)...");
        let _ = Command::new("pacman-key").args(["--init"]).status();
        let _ = Command::new("pacman-key")
            .args(["--populate", "archlinux", "cachyos", "hyperxos"])
            .status();

        // Copy gnupg keyring to target
        let _ = fs::create_dir_all("/mnt/etc/pacman.d/gnupg");
        let _ = Command::new("chmod").args(["700", "/mnt/etc/pacman.d/gnupg"]).status();
        let _ = Command::new("cp")
            .args(["-a", "/etc/pacman.d/gnupg/.", "/mnt/etc/pacman.d/gnupg/"])
            .status();

        Ok(())
    }

    /// Execute pacstrap -K /mnt <packages> streaming real-time terminal output
    pub fn run_pacstrap<F>(mut log_callback: F) -> Result<(), String>
    where
        F: FnMut(&str) + Send + 'static,
    {
        log_callback("Starting pacstrap fresh online installation to /mnt...");

        let mut cmd = Command::new("pacstrap");
        cmd.arg("-K").arg("/mnt");
        for pkg in BASE_ONLINE_PACKAGES {
            cmd.arg(pkg);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to execute pacstrap command: {}", e))?;

        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

        // Read output line by line
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            log_callback(&line);
        }

        let err_reader = BufReader::new(stderr);
        for line in err_reader.lines().flatten() {
            log_callback(&format!("[WARN] {}", line));
        }

        let status = child
            .wait()
            .map_err(|e| format!("Error waiting for pacstrap process: {}", e))?;

        if !status.success() {
            return Err("pacstrap installation failed with non-zero exit status".to_string());
        }

        log_callback("pacstrap package deployment successfully completed!");
        Ok(())
    }
}
