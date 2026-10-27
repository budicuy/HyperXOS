use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct VmConfig {
    pub ram_mb: u32,
    pub smp_cores: u32,
    pub disk_size: String,
    pub disk_image: PathBuf,
    pub ovmf_code: Option<PathBuf>,
    pub ovmf_vars_orig: Option<PathBuf>,
    pub ovmf_vars_local: PathBuf,
    pub qemu_img_bin: PathBuf,
    pub has_kvm: bool,
}

impl VmConfig {
    pub fn new(root: &Path, ram_mb: u32, smp_cores: u32) -> Self {
        let disk_image = root.join("rust/hyperx-vm/hyperxos-install.qcow2");
        let ovmf_vars_local = root.join("rust/hyperx-vm/OVMF_VARS.4m.fd");

        // 1. Detect UEFI firmware paths (Hermetic bundled first, then host fallbacks)
        let ovmf_candidates = [
            (
                root.join("rust/hyperx-vm/.bin/firmware/OVMF_CODE.4m.fd"),
                root.join("rust/hyperx-vm/.bin/firmware/OVMF_VARS.4m.fd"),
            ),
            (
                PathBuf::from("/usr/share/edk2/x64/OVMF_CODE.4m.fd"),
                PathBuf::from("/usr/share/edk2/x64/OVMF_VARS.4m.fd"),
            ),
            (
                PathBuf::from("/usr/share/edk2-ovmf/x64/OVMF_CODE.4m.fd"),
                PathBuf::from("/usr/share/edk2-ovmf/x64/OVMF_VARS.4m.fd"),
            ),
            (
                PathBuf::from("/usr/share/edk2-ovmf/x64/OVMF_CODE.fd"),
                PathBuf::from("/usr/share/edk2-ovmf/x64/OVMF_VARS.fd"),
            ),
            (
                PathBuf::from("/usr/share/OVMF/OVMF_CODE.fd"),
                PathBuf::from("/usr/share/OVMF/OVMF_VARS.fd"),
            ),
        ];

        let mut ovmf_code = None;
        let mut ovmf_vars_orig = None;

        for (code, vars) in &ovmf_candidates {
            if code.exists() && vars.exists() {
                ovmf_code = Some(code.clone());
                ovmf_vars_orig = Some(vars.clone());
                break;
            }
        }

        // 2. Detect qemu-img binary (bundled first, then host PATH)
        let bundled_qemu_img = root.join("rust/hyperx-vm/.bin/bin/qemu-img");
        let qemu_img_bin = if bundled_qemu_img.exists() {
            bundled_qemu_img
        } else {
            PathBuf::from("qemu-img")
        };

        // 3. Detect KVM Hardware Acceleration
        let kvm = Path::new("/dev/kvm");
        let mut has_kvm = false;
        if kvm.exists() {
            if let Ok(meta) = fs::metadata(kvm) {
                use std::os::unix::fs::PermissionsExt;
                let mode = meta.permissions().mode();
                // Check if readable/writable by user or group
                if (mode & 0o006 != 0) || (mode & 0o060 != 0) {
                    has_kvm = true;
                }
            }
        }

        Self {
            ram_mb,
            smp_cores,
            disk_size: "40G".to_string(),
            disk_image,
            ovmf_code,
            ovmf_vars_orig,
            ovmf_vars_local,
            qemu_img_bin,
            has_kvm,
        }
    }
}
