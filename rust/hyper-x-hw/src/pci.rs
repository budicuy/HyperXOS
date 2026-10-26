use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciDevice {
    pub slot: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u32,
    pub vendor_name: String,
    pub device_name: Option<String>,
}

impl PciDevice {
    /// Check if this PCI device is a display / graphics controller
    pub fn is_display_controller(&self) -> bool {
        // Base class 0x03 is display controller:
        // 0x0300 = VGA compatible controller
        // 0x0302 = 3D controller
        // 0x0380 = Miscellaneous display controller
        (self.class_code >> 16) == 0x03
    }

    /// Check if this device is NVIDIA
    pub fn is_nvidia(&self) -> bool {
        self.vendor_id == 0x10de
    }

    /// Check if this device is AMD
    pub fn is_amd(&self) -> bool {
        self.vendor_id == 0x1002
    }

    /// Check if this device is Intel
    pub fn is_intel(&self) -> bool {
        self.vendor_id == 0x8086
    }
}

pub fn scan_pci_devices() -> Vec<PciDevice> {
    let mut devices = Vec::new();
    let pci_path = Path::new("/sys/bus/pci/devices");

    if let Ok(entries) = fs::read_dir(pci_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let slot = entry.file_name().to_string_lossy().to_string();

            let vendor_id = read_hex_u16(&path.join("vendor"));
            let device_id = read_hex_u16(&path.join("device"));
            let class_code = read_hex_u32(&path.join("class"));

            if let (Some(vendor_id), Some(device_id), Some(class_code)) =
                (vendor_id, device_id, class_code)
            {
                let vendor_name = match vendor_id {
                    0x10de => "NVIDIA Corporation".to_string(),
                    0x8086 => "Intel Corporation".to_string(),
                    0x1002 => "Advanced Micro Devices, Inc. [AMD/ATI]".to_string(),
                    0x1af4 => "Red Hat, Inc. [Virtio]".to_string(),
                    0x15ad => "VMware".to_string(),
                    0x80ee => "Oracle Corporation [VirtualBox]".to_string(),
                    _ => format!("Vendor 0x{:04x}", vendor_id),
                };

                devices.push(PciDevice {
                    slot,
                    vendor_id,
                    device_id,
                    class_code,
                    vendor_name,
                    device_name: None,
                });
            }
        }
    }

    devices.sort_by(|a, b| a.slot.cmp(&b.slot));
    devices
}

fn read_hex_u16(path: &Path) -> Option<u16> {
    let s = fs::read_to_string(path).ok()?;
    let trimmed = s.trim().trim_start_matches("0x");
    u16::from_str_radix(trimmed, 16).ok()
}

fn read_hex_u32(path: &Path) -> Option<u32> {
    let s = fs::read_to_string(path).ok()?;
    let trimmed = s.trim().trim_start_matches("0x");
    u32::from_str_radix(trimmed, 16).ok()
}
