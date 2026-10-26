use serde::{Deserialize, Serialize};

use crate::pci::PciDevice;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NvidiaArch {
    TuringOrNewer,
    PascalOrMaxwell,
    KeplerOrOlder,
}

impl NvidiaArch {
    pub fn display_name(&self) -> &'static str {
        match self {
            NvidiaArch::TuringOrNewer => "Turing / Ampere / Ada Lovelace / Blackwell (Modern Open Driver)",
            NvidiaArch::PascalOrMaxwell => "Pascal / Maxwell (Legacy Legacy Architecture)",
            NvidiaArch::KeplerOrOlder => "Kepler or Older (Vintage Architecture)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDiagnostic {
    pub total_gpus: usize,
    pub has_nvidia: bool,
    pub has_amd: bool,
    pub has_intel: bool,
    pub is_hybrid_laptop: bool,
    pub nvidia_arch: Option<NvidiaArch>,
    pub detected_cards: Vec<PciDevice>,
    pub recommended_packages: Vec<String>,
    pub kernel_modules_early_kms: Vec<String>,
    pub kernel_cmdline_params: Vec<String>,
    pub enable_services: Vec<String>,
}

pub fn analyze_gpus(pci_devices: &[PciDevice]) -> GpuDiagnostic {
    let display_cards: Vec<PciDevice> = pci_devices
        .iter()
        .filter(|d| d.is_display_controller())
        .cloned()
        .collect();

    let total_gpus = display_cards.len();
    let mut has_nvidia = false;
    let mut has_amd = false;
    let mut has_intel = false;
    let mut nvidia_arch = None;

    let mut pkgs = Vec::new();
    let mut kms_modules = Vec::new();
    let mut cmdline_params = Vec::new();
    let mut services = Vec::new();

    for card in &display_cards {
        if card.is_nvidia() {
            has_nvidia = true;
            let arch = classify_nvidia_device(card.device_id);
            nvidia_arch = Some(arch.clone());

            match arch {
                NvidiaArch::TuringOrNewer => {
                    pkgs.push("nvidia-open".to_string());
                    pkgs.push("nvidia-utils".to_string());
                    pkgs.push("opencl-nvidia".to_string());

                    kms_modules.push("nvidia".to_string());
                    kms_modules.push("nvidia_modeset".to_string());
                    kms_modules.push("nvidia_uvm".to_string());
                    kms_modules.push("nvidia_drm".to_string());

                    cmdline_params.push("nvidia_drm.modeset=1".to_string());
                    cmdline_params.push("nvidia_drm.fbdev=1".to_string());
                }
                NvidiaArch::PascalOrMaxwell | NvidiaArch::KeplerOrOlder => {
                    // Modern Wayland operates best with Nouveau on legacy Pascal/Maxwell/Kepler
                    pkgs.push("mesa".to_string());
                }
            }
        }

        if card.is_amd() {
            has_amd = true;
            pkgs.push("mesa".to_string());
            pkgs.push("vulkan-radeon".to_string());
        }

        if card.is_intel() {
            has_intel = true;
            pkgs.push("mesa".to_string());
            pkgs.push("vulkan-intel".to_string());
            pkgs.push("intel-media-driver".to_string());
        }
    }

    // Hybrid Laptop Dual-GPU detection
    let is_hybrid_laptop = total_gpus >= 2 && has_nvidia && (has_intel || has_amd);
    if is_hybrid_laptop {
        pkgs.push("switcheroo-control".to_string());
        pkgs.push("nvidia-prime".to_string());
        services.push("switcheroo-control.service".to_string());
    }

    // Remove duplicates
    pkgs.sort();
    pkgs.dedup();
    kms_modules.dedup();
    cmdline_params.dedup();
    services.dedup();

    GpuDiagnostic {
        total_gpus,
        has_nvidia,
        has_amd,
        has_intel,
        is_hybrid_laptop,
        nvidia_arch,
        detected_cards: display_cards,
        recommended_packages: pkgs,
        kernel_modules_early_kms: kms_modules,
        kernel_cmdline_params: cmdline_params,
        enable_services: services,
    }
}

fn classify_nvidia_device(dev_id: u16) -> NvidiaArch {
    // Pascal: 0x1b00..=0x1dff
    // Maxwell: 0x1300..=0x17ff
    if (0x1b00..=0x1dff).contains(&dev_id) || (0x1300..=0x17ff).contains(&dev_id) {
        return NvidiaArch::PascalOrMaxwell;
    }

    // Kepler: 0x0fc0..=0x12ff
    if (0x0fc0..=0x12ff).contains(&dev_id) {
        return NvidiaArch::KeplerOrOlder;
    }

    // Turing (0x1e00..=0x21ff, 0x1f00..=0x1fff), Ampere (0x2200..=0x26ff),
    // Ada Lovelace (0x2700..=0x28ff), Blackwell (0x2900+)
    NvidiaArch::TuringOrNewer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_nvidia_devices() {
        // GTX 1660 SUPER (Turing)
        assert_eq!(classify_nvidia_device(0x21c4), NvidiaArch::TuringOrNewer);
        // RTX 3080 (Ampere)
        assert_eq!(classify_nvidia_device(0x2206), NvidiaArch::TuringOrNewer);
        // RTX 4090 (Ada Lovelace)
        assert_eq!(classify_nvidia_device(0x2684), NvidiaArch::TuringOrNewer);
        // GTX 1080 (Pascal)
        assert_eq!(classify_nvidia_device(0x1b80), NvidiaArch::PascalOrMaxwell);
        // GTX 970 (Maxwell)
        assert_eq!(classify_nvidia_device(0x13c2), NvidiaArch::PascalOrMaxwell);
        // GTX 760 (Kepler)
        assert_eq!(classify_nvidia_device(0x1187), NvidiaArch::KeplerOrOlder);
    }

    #[test]
    fn test_analyze_hybrid_laptop() {
        let intel_card = PciDevice {
            slot: "0000:00:02.0".to_string(),
            vendor_id: 0x8086,
            device_id: 0x9bc4,
            class_code: 0x030000,
            vendor_name: "Intel Corporation".to_string(),
            device_name: Some("CometLake-H GT2 [UHD Graphics]".to_string()),
        };
        let nvidia_card = PciDevice {
            slot: "0000:01:00.0".to_string(),
            vendor_id: 0x10de,
            device_id: 0x2560, // RTX 3060 Mobile
            class_code: 0x030000,
            vendor_name: "NVIDIA Corporation".to_string(),
            device_name: Some("GA106M [GeForce RTX 3060 Mobile / Max-Q]".to_string()),
        };

        let diag = analyze_gpus(&[intel_card, nvidia_card]);
        assert_eq!(diag.total_gpus, 2);
        assert!(diag.has_nvidia);
        assert!(diag.has_intel);
        assert!(diag.is_hybrid_laptop);
        assert!(diag.recommended_packages.contains(&"switcheroo-control".to_string()));
        assert!(diag.recommended_packages.contains(&"nvidia-open".to_string()));
        assert!(diag.recommended_packages.contains(&"vulkan-intel".to_string()));
    }
}
