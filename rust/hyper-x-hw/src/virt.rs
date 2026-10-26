use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VirtType {
    QemuKvm,
    VirtualBox,
    VMware,
    None,
}

impl VirtType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VirtType::QemuKvm => "kvm",
            VirtType::VirtualBox => "oracle",
            VirtType::VMware => "vmware",
            VirtType::None => "none",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            VirtType::QemuKvm => "QEMU / KVM Hypervisor",
            VirtType::VirtualBox => "Oracle VirtualBox",
            VirtType::VMware => "VMware Workstation / ESXi",
            VirtType::None => "Physical Hardware (Bare Metal)",
        }
    }

    pub fn required_packages(&self) -> Vec<&'static str> {
        match self {
            VirtType::QemuKvm => vec!["qemu-guest-agent"],
            VirtType::VirtualBox => vec!["virtualbox-guest-utils-nox"],
            VirtType::VMware => vec!["open-vm-tools"],
            VirtType::None => vec![],
        }
    }

    pub fn required_services(&self) -> Vec<&'static str> {
        match self {
            VirtType::QemuKvm => vec!["qemu-guest-agent.service"],
            VirtType::VirtualBox => vec!["vboxservice.service"],
            VirtType::VMware => vec!["vmtoolsd.service"],
            VirtType::None => vec![],
        }
    }
}

pub fn detect_virtualization() -> VirtType {
    // 1. Try systemd-detect-virt if available
    if let Ok(output) = Command::new("systemd-detect-virt").output() {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
            match v.as_str() {
                "kvm" | "qemu" | "bochs" => return VirtType::QemuKvm,
                "oracle" | "virtualbox" => return VirtType::VirtualBox,
                "vmware" => return VirtType::VMware,
                "none" => return VirtType::None,
                _ => {}
            }
        }
    }

    // 2. Direct DMI inspect fallback
    let sys_vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor")
        .unwrap_or_default()
        .to_lowercase();
    let product_name = fs::read_to_string("/sys/class/dmi/id/product_name")
        .unwrap_or_default()
        .to_lowercase();

    if sys_vendor.contains("qemu")
        || sys_vendor.contains("bochs")
        || product_name.contains("standard pc")
        || product_name.contains("qemu")
    {
        return VirtType::QemuKvm;
    }

    if sys_vendor.contains("innotek") || product_name.contains("virtualbox") {
        return VirtType::VirtualBox;
    }

    if sys_vendor.contains("vmware") || product_name.contains("vmware") {
        return VirtType::VMware;
    }

    VirtType::None
}
