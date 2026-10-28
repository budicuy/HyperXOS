mod build;
mod check;
mod checksum;
mod clean;
mod qa;
mod release;
mod repo;
mod setup;
mod util;
mod vm;

use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;

use crate::util::{get_project_root, run_stream};

#[derive(Parser)]
#[command(name = "hyperx")]
#[command(author = "HyperXOS Development Team <dev@hyperxos.org>")]
#[command(version = "1.0.0")]
#[command(about = "Unified Pure Rust Developer & Build Automation Toolchain for HyperXOS Linux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup host build dependencies, cache directories, and permissions
    Setup,
    /// Validate host build dependencies, KVM hardware acceleration, and configurations
    Check,
    /// Build or synchronize local pacman repository database
    Repo {
        #[arg(default_value = "update")]
        action: String,
    },
    /// Build HyperXOS ISO (defaults to fast development mode, use --prod for release)
    Build {
        /// Build in production mode (ultra compression, clean release, size enforcement)
        #[arg(long, short)]
        prod: bool,
    },
    /// Alias for building HyperXOS ISO in production mode (zstd -22, <2.0GB enforcement)
    Production,
    /// Alias for building HyperXOS ISO in production mode
    #[command(name = "build-production")]
    BuildProduction,
    /// Run HyperXOS in QEMU virtual machine test harness (dev, install, run)
    Vm {
        /// Mode to run: 'dev' (live ISO), 'install' (clean 40G disk), or 'run' (boot installed OS)
        #[arg(default_value = "dev")]
        mode: String,
    },
    /// Run fresh QEMU installation test on 40GB virtual disk
    Install,
    /// Alias for 'install'
    #[command(name = "vm-install")]
    VmInstall,
    /// Run installed HyperXOS system from 40GB virtual disk
    Run,
    /// Run live ISO in QEMU without formatting virtual disk
    Dev,
    /// Run automated QA testing and 25s non-interactive headless smoke test
    Test,
    /// Clean temporary build directories and artifacts
    Clean {
        /// Also remove output ISOs and virtual test disk images
        #[arg(long, short)]
        all: bool,
    },
    /// Generate official SHA256 and BLAKE2b (b2sum) checksums for built ISOs
    Checksum,
    /// Prepare and publish GitHub release with git tag and ISO assets
    Release,
    /// Launch HyperXTools GUI application directly
    Tools,
    /// Alias for 'tools' (launches HyperXTools)
    Hyperx,
    /// Launch HyperXInstaller GUI application directly
    Installer,
    /// Run Hardware & GPU Diagnostic Engine (hyper-x-hw)
    Hw {
        #[arg(default_value = "detect")]
        action: String,
    },
    /// Run Pure Rust Direct-GPU Virtual Environment Runner (hyperx-vm)
    #[command(name = "hyperx-vm")]
    HyperxVm {
        /// Mode to run: 'dev' / 'live', 'install', 'run', or 'clean'
        #[arg(default_value = "live")]
        mode: String,
    },
    /// Install 'hyperx' CLI binary to /usr/local/bin or ~/.cargo/bin
    InstallCli,
}

fn run_cargo_app(bin_name: &str, extra_args: &[&str]) -> Result<(), String> {
    let root = get_project_root();
    let manifest = root.join("rust/Cargo.toml");
    if !manifest.exists() {
        return Err(format!("Manifest not found at {}", manifest.display()));
    }

    let mut cmd = Command::new("cargo");
    cmd.args([
        "run",
        "--release",
        "--manifest-path",
        manifest.to_str().unwrap(),
        "--bin",
        bin_name,
    ]);
    if !extra_args.is_empty() {
        cmd.arg("--");
        for a in extra_args {
            cmd.arg(a);
        }
    }

    run_stream(&mut cmd, bin_name)
}

fn install_cli_binary(root: &Path) -> Result<(), String> {
    println!("==> [HyperXOS] Installing 'hyperx' CLI binary to system...");
    let cli_bin = root.join("rust/target/release/hyperx");
    if !cli_bin.exists() {
        println!("  -> Building release binary...");
        let mut build_cmd = Command::new("cargo");
        build_cmd.args([
            "build",
            "--release",
            "--manifest-path",
            root.join("rust/Cargo.toml").to_str().unwrap(),
            "--bin",
            "hyperx",
        ]);
        run_stream(&mut build_cmd, "cargo build hyperx")?;
    }

    let dest = Path::new("/usr/local/bin/hyperx");
    println!("  -> Installing to {} (requires sudo)...", dest.display());
    let mut cp_cmd = Command::new("sudo");
    cp_cmd.args(["install", "-m", "755", cli_bin.to_str().unwrap(), dest.to_str().unwrap()]);
    run_stream(&mut cp_cmd, "install hyperx")?;

    println!("[✓] 'hyperx' command installed successfully! You can now run 'hyperx' anywhere in your terminal.");
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let root = get_project_root();

    let res = match cli.command {
        Commands::Setup => setup::run_setup(&root),
        Commands::Check => check::run_check(&root),
        Commands::Repo { action } => repo::run_repo(&root, &action),
        Commands::Build { prod } => build::run_build(&root, prod),
        Commands::Production | Commands::BuildProduction => build::run_build(&root, true),
        Commands::Vm { mode } => vm::run_vm(&root, &mode),
        Commands::Install | Commands::VmInstall => vm::run_vm(&root, "install"),
        Commands::Run => vm::run_vm(&root, "run"),
        Commands::Dev => vm::run_vm(&root, "dev"),
        Commands::Test => qa::run_test(&root),
        Commands::Clean { all } => clean::run_clean(&root, all),
        Commands::Checksum => checksum::run_checksum(&root),
        Commands::Release => release::run_release(&root),
        Commands::Tools | Commands::Hyperx => run_cargo_app("hyper-x-tools", &[]),
        Commands::Installer => run_cargo_app("hyper-x-installer", &[]),
        Commands::Hw { action } => run_cargo_app("hyper-x-hw", &[&action]),
        Commands::HyperxVm { mode } => vm::run_vm(&root, &mode),
        Commands::InstallCli => install_cli_binary(&root),
    };

    if let Err(e) = res {
        eprintln!("\n[-] HyperX CLI Error: {}", e);
        std::process::exit(1);
    }
}
