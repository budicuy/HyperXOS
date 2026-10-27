mod config;
mod display;
mod engine;
mod provisioner;
mod storage;
mod util;

use clap::{Parser, Subcommand};
use std::path::Path;
use std::process;

use crate::config::VmConfig;
use crate::display::DisplayManager;
use crate::engine::VmEngine;
use crate::provisioner::VmmProvisioner;
use crate::storage::StorageManager;
use crate::util::get_project_root;

#[derive(Parser)]
#[command(name = "hyperx-vm")]
#[command(author = "HyperXOS Development Team <dev@hyperxos.org>")]
#[command(version = "1.0.0")]
#[command(about = "Pure Rust Real Hardware-Accelerated Virtual Machine Manager for HyperXOS")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// RAM allocation in Megabytes
    #[arg(long, default_value_t = 4096)]
    ram: u32,

    /// Number of virtual CPU cores
    #[arg(long, default_value_t = 4)]
    cores: u32,
}

#[derive(Subcommand)]
enum Commands {
    /// Boot HyperXOS Live ISO via UEFI Firmware & Limine Bootloader
    Live,
    /// Alias for 'live'
    Dev,
    /// Run fresh installation environment targeting 40GB virtual disk
    Install,
    /// Boot installed HyperXOS system directly from 40GB virtual disk
    Run,
    /// Validate host virtualization environment (KVM, OVMF, GL display, Hermetic VMM)
    Check,
    /// Provision or update the hermetic pinned VMM binary in .bin/
    Provision,
    /// Clean temporary sockets and runtime lockfiles
    Clean,
}

fn run_check(root: &Path, config: &VmConfig) -> Result<(), String> {
    println!("==> [hyperx-vm] Checking host virtualization capabilities...");

    // 1. Hermetic VMM binary verification
    match VmmProvisioner::ensure_vmm(root) {
        Ok(bin) => println!("  [✓] Hermetic Pinned VMM verified: {}", bin.display()),
        Err(e) => return Err(format!("VMM Check Failed: {}", e)),
    }

    // 2. KVM acceleration
    if config.has_kvm {
        println!("  [✓] Hardware Virtualization Acceleration (/dev/kvm) accessible.");
    } else {
        println!("  [!] /dev/kvm is not accessible. Virtualization will run with reduced performance.");
    }

    // 3. UEFI OVMF Firmware
    if let Some(ref ovmf) = config.ovmf_code {
        println!("  [✓] UEFI Firmware found: {}", ovmf.display());
    } else {
        return Err("UEFI firmware (OVMF_CODE.4m.fd) not found in system paths.".to_string());
    }

    // 4. Display & GPU Acceleration
    let (backend, _) = DisplayManager::detect();
    println!("  [✓] Detected Display Acceleration Backend: {:?}", backend);

    println!("[✓] All virtualization checks passed successfully.");
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let root = get_project_root();
    let config = VmConfig::new(&root, cli.ram, cli.cores);

    let res = match cli.command {
        Commands::Live | Commands::Dev => VmEngine::run_live(&root, &config),
        Commands::Install => VmEngine::run_install(&root, &config),
        Commands::Run => VmEngine::run_installed(&root, &config),
        Commands::Check => run_check(&root, &config),
        Commands::Provision => VmmProvisioner::ensure_vmm(&root).map(|bin| {
            println!("[✓] Hermetic VMM binary is up-to-date and ready at: {}", bin.display());
        }),
        Commands::Clean => {
            StorageManager::clean_temp_files();
            StorageManager::reset_nvram(&config);
            println!("[✓] hyperx-vm cleaned up successfully.");
            Ok(())
        }
    };

    if let Err(e) = res {
        eprintln!("\n[-] hyperx-vm error: {}", e);
        process::exit(1);
    }
}
