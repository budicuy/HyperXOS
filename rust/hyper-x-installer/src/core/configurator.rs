use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;
use crate::core::filesystem::FilesystemContext;

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub root_password: String,
    pub hostname: String,
    pub timezone: String,
    pub keymap: String,
    pub locale: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: "hyperx".to_string(),
            fullname: "HyperX User".to_string(),
            password: "password".to_string(),
            root_password: "password".to_string(),
            hostname: "hyperxos".to_string(),
            timezone: "Asia/Jakarta".to_string(),
            keymap: "us".to_string(),
            locale: "en_US.UTF-8".to_string(),
        }
    }
}

pub struct SystemConfigurator;

impl SystemConfigurator {
    /// Configure target system settings: fstab, users, network, timezone, locale, SDDM, and mkinitcpio
    pub fn configure_system<F>(
        fs_ctx: &FilesystemContext,
        cfg: &UserConfig,
        mut progress_callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(&str),
    {
        // 1. Generate /etc/fstab
        progress_callback("Generating /etc/fstab with Btrfs subvolumes and 4GB ESP...");
        Self::generate_fstab(fs_ctx)?;

        // 2. Set Hostname & /etc/hosts
        progress_callback(&format!("Setting hostname '{}'...", cfg.hostname));
        fs::write("/mnt/etc/hostname", format!("{}\n", cfg.hostname))
            .map_err(|e| format!("Failed to write /mnt/etc/hostname: {}", e))?;

        let hosts_content = format!(
            "127.0.0.1   localhost\n::1         localhost\n127.0.1.1   {}.localdomain {}\n",
            cfg.hostname, cfg.hostname
        );
        fs::write("/mnt/etc/hosts", hosts_content)
            .map_err(|e| format!("Failed to write /mnt/etc/hosts: {}", e))?;

        // 3. Timezone & RTC
        progress_callback(&format!("Configuring timezone '{}'...", cfg.timezone));
        let tz_src = format!("/usr/share/zoneinfo/{}", cfg.timezone);
        let tz_dst = "/mnt/etc/localtime";
        let _ = fs::remove_file(tz_dst);
        let _ = symlink(&tz_src, tz_dst);
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "hwclock", "--systohc"])
            .status();

        // 4. Locales & Keymap
        progress_callback(&format!("Configuring locale '{}' and keymap '{}'...", cfg.locale, cfg.keymap));
        let locale_entry = if cfg.locale.contains("UTF-8") {
            format!("{} UTF-8\n", cfg.locale.replace(" UTF-8", ""))
        } else {
            format!("{} UTF-8\n", cfg.locale)
        };
        let _ = fs::write("/mnt/etc/locale.gen", format!("en_US.UTF-8 UTF-8\n{}", locale_entry));
        let _ = Command::new("arch-chroot").args(["/mnt", "locale-gen"]).status();
        let _ = fs::write("/mnt/etc/locale.conf", format!("LANG={}\n", cfg.locale));
        let _ = fs::write("/mnt/etc/vconsole.conf", format!("KEYMAP={}\n", cfg.keymap));

        // 5. Machine ID
        progress_callback("Generating unique machine-id...");
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "systemd-machine-id-setup"])
            .status();

        // 6. User and Root Accounts
        progress_callback(&format!("Creating user account '{}' and configuring privileges...", cfg.username));
        // Set root password
        let root_passwd_cmd = format!("echo 'root:{}' | chpasswd", cfg.root_password);
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "sh", "-c", &root_passwd_cmd])
            .status();

        // Add user with wheel & audio/video groups
        let _ = Command::new("arch-chroot")
            .args([
                "/mnt",
                "useradd",
                "-m",
                "-g",
                "users",
                "-G",
                "wheel,video,audio,storage,network,power",
                "-s",
                "/bin/bash",
                &cfg.username,
            ])
            .status();

        // Set user password
        let user_passwd_cmd = format!("echo '{}:{}' | chpasswd", cfg.username, cfg.password);
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "sh", "-c", &user_passwd_cmd])
            .status();

        // Set full name
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "chfn", "-f", &cfg.fullname, &cfg.username])
            .status();

        // Enable wheel in sudoers
        let _ = fs::create_dir_all("/mnt/etc/sudoers.d");
        let sudo_rule = "%wheel ALL=(ALL:ALL) ALL\n";
        let _ = fs::write("/mnt/etc/sudoers.d/10-installer-wheel", sudo_rule);
        let _ = Command::new("chmod")
            .args(["440", "/mnt/etc/sudoers.d/10-installer-wheel"])
            .status();

        // 7. SDDM & Wayland Configuration
        progress_callback("Configuring SDDM Wayland display manager and systemd services...");
        let _ = fs::create_dir_all("/mnt/etc/sddm.conf.d");
        let sddm_cfg = r#"[General]
DisplayServer=wayland
GreeterEnvironment=QT_WAYLAND_SHELL_INTEGRATION=layer-shell

[Wayland]
CompositorCommand=Hyprland
"#;
        let _ = fs::write("/mnt/etc/sddm.conf.d/10-wayland.conf", sddm_cfg);

        let autologin_cfg = format!(
            r#"[Autologin]
User={}
Session=hyprland
Relogin=true
"#,
            cfg.username
        );
        let _ = fs::write("/mnt/etc/sddm.conf.d/autologin.conf", autologin_cfg);

        // DNS Configuration
        let _ = fs::write(
            "/mnt/etc/resolv.conf",
            "nameserver 1.1.1.1\nnameserver 8.8.8.8\nnameserver 1.0.0.1\n",
        );

        // Enable essential system services natively
        for srv in [
            "sddm.service",
            "NetworkManager.service",
            "systemd-resolved.service",
            "systemd-timesyncd.service",
            "bluetooth.service",
            "qemu-guest-agent.service",
        ] {
            let _ = Command::new("arch-chroot")
                .args(["/mnt", "systemctl", "enable", srv])
                .status();
        }

        let _ = Command::new("arch-chroot")
            .args(["/mnt", "systemctl", "set-default", "graphical.target"])
            .status();

        // 8. User Desktop Shortcuts & Keyring Setup
        let user_desktop_dir = format!("/mnt/home/{}/Desktop", cfg.username);
        let _ = fs::create_dir_all(&user_desktop_dir);
        let user_tools_desktop = format!("{}/hyper-x-tools.desktop", user_desktop_dir);
        if Path::new("/mnt/usr/share/applications/hyper-x-tools.desktop").exists() {
            let _ = fs::copy("/mnt/usr/share/applications/hyper-x-tools.desktop", &user_tools_desktop);
            let _ = Command::new("chmod").args(["755", &user_tools_desktop]).status();
            let chown_arg = format!("{}:{}", cfg.username, cfg.username);
            let _ = Command::new("arch-chroot")
                .args(["/mnt", "chown", "-R", &chown_arg, &format!("/home/{}/Desktop", cfg.username)])
                .status();
        }

        // Initialize clean pacman keyring in target
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "pacman-key", "--init"])
            .status();
        let _ = Command::new("arch-chroot")
            .args(["/mnt", "pacman-key", "--populate", "archlinux", "cachyos", "hyperxos"])
            .status();

        // 9. Hardware & GPU Detection via Pure Rust hyper-x-hw
        progress_callback("Executing native hardware & GPU driver auto-configuration (hyper-x-hw)...");
        if Path::new("/usr/bin/hyper-x-hw").exists() {
            let _ = Command::new("/usr/bin/hyper-x-hw")
                .args(["apply", "--target", "/mnt", "--no-pkg"])
                .status();
        } else if Path::new("/mnt/usr/bin/hyper-x-hw").exists() {
            let _ = Command::new("arch-chroot")
                .args(["/mnt", "/usr/bin/hyper-x-hw", "apply", "--no-pkg"])
                .status();
        } else if fs::metadata("/mnt/usr/local/bin/hw-detect-gpu").is_ok() {
            let _ = Command::new("arch-chroot")
                .args(["/mnt", "/usr/local/bin/hw-detect-gpu", "/"])
                .status();
        }

        // 9. Rebuild mkinitcpio for target kernel (ensuring btrfs module is included)
        progress_callback("Configuring mkinitcpio modules and generating initramfs with mkinitcpio -P...");
        let _ = fs::create_dir_all("/mnt/etc/mkinitcpio.conf.d");
        let _ = fs::write("/mnt/etc/mkinitcpio.conf.d/hyperxos.conf", "MODULES+=(btrfs)\n");
        let mkinit_res = Command::new("arch-chroot")
            .args(["/mnt", "mkinitcpio", "-P"])
            .status()
            .map_err(|e| format!("Failed to run mkinitcpio: {}", e))?;

        if !mkinit_res.success() {
            return Err("mkinitcpio initramfs generation returned an error".to_string());
        }

        progress_callback("Target system configuration completed successfully.");
        Ok(())
    }

    fn generate_fstab(fs_ctx: &FilesystemContext) -> Result<(), String> {
        let mut fstab = String::from("# Static File System Information (HyperXOS)\n# <file system> <mount point> <type> <options> <dump> <pass>\n\n");

        // Root @ subvolume
        fstab.push_str(&format!(
            "UUID={:<36}  /      btrfs   rw,noatime,compress=zstd:1,space_cache=v2,subvol=@      0 0\n",
            fs_ctx.root_uuid
        ));

        // Home @home subvolume
        fstab.push_str(&format!(
            "UUID={:<36}  /home  btrfs   rw,noatime,compress=zstd:1,space_cache=v2,subvol=@home  0 0\n",
            fs_ctx.root_uuid
        ));

        // ESP Boot partition (4GB FAT32)
        fstab.push_str(&format!(
            "UUID={:<36}  /boot  vfat    rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=ascii,shortname=mixed,utf8,errors=remount-ro  0 2\n",
            fs_ctx.boot_uuid
        ));

        fs::write("/mnt/etc/fstab", fstab)
            .map_err(|e| format!("Failed to write /mnt/etc/fstab: {}", e))?;

        Ok(())
    }
}
