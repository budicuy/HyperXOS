pub mod config;
pub mod gpu;
pub mod pci;
pub mod virt;

use std::env;
use std::path::PathBuf;

fn print_help() {
    println!("HyperXOS Multi-Tier Hardware & GPU Detection Engine (Pure Rust)");
    println!("Usage:");
    println!("  hyper-x-hw detect [--json]         Scan and display hardware diagnostic report");
    println!("  hyper-x-hw apply [--target <dir>]  Configure drivers and bootloader parameters");
    println!("  hyper-x-hw --help                  Show this help message");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let action = args.get(1).map(|s| s.as_str()).unwrap_or("detect");

    match action {
        "detect" => {
            let json_mode = args.iter().any(|a| a == "--json");
            let pci_devs = pci::scan_pci_devices();
            let virt = virt::detect_virtualization();
            let gpu_diag = gpu::analyze_gpus(&pci_devs);

            if json_mode {
                let report = serde_json::json!({
                    "virtualization": virt,
                    "gpu": gpu_diag,
                });
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                hyperx_ui_kit::print_cli_banner(
                    "HyperXOS Hardware & Graphics Diagnostic Engine (Pure Rust)",
                    None,
                );
                println!("Virtualization: {}", virt.display_name());
                println!("Detected Display GPUs: {}", gpu_diag.total_gpus);
                for card in &gpu_diag.detected_cards {
                    println!("  -> [{}] {} (PCI ID 0x{:04x}:0x{:04x})", card.slot, card.vendor_name, card.vendor_id, card.device_id);
                }
                if let Some(ref arch) = gpu_diag.nvidia_arch {
                    println!("NVIDIA Architecture: {}", arch.display_name());
                }
                if gpu_diag.is_hybrid_laptop {
                    println!("Laptop Configuration: Dual-GPU Hybrid Optimus / PRIME Detected");
                }
                println!("\nRecommended Driver Stack:");
                println!("  Packages: {:?}", gpu_diag.recommended_packages);
                println!("  Early KMS Modules: {:?}", gpu_diag.kernel_modules_early_kms);
                println!("  Kernel Parameters: {:?}", gpu_diag.kernel_cmdline_params);
                println!("  Systemd Services:  {:?}", gpu_diag.enable_services);
                println!("============================================================");
            }
        }
        "apply" => {
            let mut target_dir = PathBuf::from("/");
            let mut install_pkgs = true;

            let mut i = 2;
            while i < args.len() {
                if args[i] == "--target" && i + 1 < args.len() {
                    target_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                } else if args[i] == "--no-pkg" {
                    install_pkgs = false;
                }
                i += 1;
            }

            let pci_devs = pci::scan_pci_devices();
            let virt = virt::detect_virtualization();
            let gpu_diag = gpu::analyze_gpus(&pci_devs);

            let applier = config::ConfigApplier::new(&target_dir);
            if let Err(e) = applier.apply_all(&virt, &gpu_diag, install_pkgs) {
                eprintln!("[-] Error applying hardware configuration: {}", e);
                std::process::exit(1);
            }
        }
        "--help" | "-h" => {
            print_help();
        }
        _ => {
            print_help();
            std::process::exit(1);
        }
    }

    Ok(())
}
