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
use std::process::Command;

use crate::util::{get_project_root, run_stream};

#[derive(Parser)]
#[command(name = "hyperx")]
#[command(author = "HyperXOS Development Team <dev@hyperxos.org>")]
#[command(version = "2.2.0")]
#[command(about = "Unified Pure Rust Developer & Build Automation Toolchain for HyperXOS Linux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // ========================================================================
    // Core Workflow Commands (Visible in `hyperx --help`)
    // ========================================================================

    /// Build HyperXOS packages, repo, ISO image, and checksums
    Build {
        /// Build in production mode (ultra compression, clean release, size enforcement)
        #[arg(long, short)]
        prod: bool,

        /// Only build packages and local repo database (skip ISO image creation)
        #[arg(long)]
        repo_only: bool,
    },

    /// Run automated QA testing and 25s non-interactive headless smoke test
    Test,

    /// Run HyperXOS in QEMU virtual machine test harness (install, run, clean)
    Vm {
        #[command(subcommand)]
        action: VmAction,
    },

    /// Manage local pacman repository database (build, sync, check)
    Repo {
        #[command(subcommand)]
        action: Option<repo::RepoAction>,
    },

    /// Launch HyperXOS internal applications directly (tools, installer, hw)
    #[command(alias = "app")]
    Launch {
        #[command(subcommand)]
        app: AppTarget,
    },

    /// Clean temporary build directories and artifacts
    Clean {
        /// Also remove output ISOs and virtual test disk images
        #[arg(long, short)]
        all: bool,
    },

    /// Validate host build dependencies, KVM hardware acceleration, and configurations
    Check,

    /// Setup host build dependencies, cache directories, and permissions
    Setup {
        /// Also install 'hyperx' CLI launcher to /usr/local/bin
        #[arg(long)]
        cli: bool,
    },

    /// Prepare and publish GitHub release with git tag and ISO assets
    Release {
        /// Only compute official SHA256 and BLAKE2b checksums without publishing
        #[arg(long)]
        checksum_only: bool,
    },

    // ========================================================================
    // Ergonomic Shortcuts & Legacy Aliases (Hidden to maintain a clean --help)
    // ========================================================================

    /// Shortcut for 'vm install' (run fresh QEMU installation test on 40GB virtual disk)
    #[command(hide = true)]
    Install {
        /// Run non-interactive automated installation test via QEMU guest agent
        #[arg(long)]
        auto: bool,
    },

    /// Legacy alias for 'vm install'
    #[command(hide = true, name = "vm-install")]
    VmInstall {
        /// Run non-interactive automated installation test via QEMU guest agent
        #[arg(long)]
        auto: bool,
    },

    /// Shortcut for 'vm run' (run installed HyperXOS system from 40GB virtual disk)
    #[command(hide = true)]
    Run {
        /// Run automated smoke test
        #[arg(long)]
        auto: bool,
    },

    /// Legacy alias for 'build --prod'
    #[command(hide = true)]
    Production,

    /// Legacy alias for 'build --prod'
    #[command(hide = true, name = "build-production")]
    BuildProduction,

    /// Legacy alias for 'vm'
    #[command(hide = true, name = "hyperx-vm")]
    HyperxVm {
        /// Mode to run: 'dev' / 'live', 'install', 'run', or 'clean'
        #[arg(default_value = "live")]
        mode: String,
    },

    /// Legacy shortcut for 'launch tools'
    #[command(hide = true)]
    Tools,

    /// Legacy shortcut for 'launch tools'
    #[command(hide = true)]
    Hyperx,

    /// Legacy shortcut for 'launch installer'
    #[command(hide = true)]
    Installer,

    /// Legacy shortcut for 'launch hw'
    #[command(hide = true)]
    Hw {
        #[arg(default_value = "detect")]
        action: String,
    },

    /// Legacy shortcut for 'release --checksum-only'
    #[command(hide = true)]
    Checksum,

    /// Legacy shortcut for 'setup --cli'
    #[command(hide = true, name = "install-cli")]
    InstallCli,
}

#[derive(Subcommand, Debug, Clone)]
pub enum VmAction {
    /// Run fresh QEMU installation test on 40GB virtual disk
    Install {
        /// Run non-interactive automated installation test via QEMU guest agent
        #[arg(long)]
        auto: bool,
    },

    /// Run installed HyperXOS system from 40GB virtual disk
    Run {
        /// Run automated smoke test
        #[arg(long)]
        auto: bool,
    },

    /// Clean virtual machine disk images and runtime lockfiles
    Clean,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AppTarget {
    /// Launch HyperXTools Control Center & Hardware Monitor
    Tools,

    /// Launch HyperXOS Graphical Installer directly on host
    Installer,

    /// Run Hardware & GPU Diagnostic Engine (hyper-x-hw)
    Hw {
        #[arg(default_value = "detect")]
        action: String,
    },
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

fn main() {
    let cli = Cli::parse();
    let root = get_project_root();

    let res = match cli.command {
        // Core commands
        Commands::Build { prod, repo_only } => build::run_build(&root, prod, repo_only),
        Commands::Test => qa::run_test(&root),
        Commands::Vm { action } => match action {
            VmAction::Install { auto } => vm::run_vm(&root, "install", auto),
            VmAction::Run { auto } => vm::run_vm(&root, "run", auto),
            VmAction::Clean => vm::run_vm(&root, "clean", false),
        },
        Commands::Repo { action } => match action {
            Some(act) => repo::run_repo_action(&root, &act),
            None => repo::run_repo(&root, "update"),
        },
        Commands::Launch { app } => match app {
            AppTarget::Tools => run_cargo_app("hyper-x-tools", &[]),
            AppTarget::Installer => run_cargo_app("hyper-x-installer", &[]),
            AppTarget::Hw { action } => run_cargo_app("hyper-x-hw", &[&action]),
        },
        Commands::Clean { all } => clean::run_clean(&root, all),
        Commands::Check => check::run_check(&root),
        Commands::Setup { cli } => setup::run_setup(&root, cli),
        Commands::Release { checksum_only } => release::run_release(&root, checksum_only),

        // Hidden ergonomic shortcuts & legacy aliases
        Commands::Install { auto } | Commands::VmInstall { auto } => {
            vm::run_vm(&root, "install", auto)
        }
        Commands::Run { auto } => vm::run_vm(&root, "run", auto),
        Commands::Production | Commands::BuildProduction => build::run_build(&root, true, false),
        Commands::HyperxVm { mode } => vm::run_vm(&root, &mode, false),
        Commands::Tools | Commands::Hyperx => run_cargo_app("hyper-x-tools", &[]),
        Commands::Installer => run_cargo_app("hyper-x-installer", &[]),
        Commands::Hw { action } => run_cargo_app("hyper-x-hw", &[&action]),
        Commands::Checksum => checksum::run_checksum(&root),
        Commands::InstallCli => setup::install_cli_binary(&root),
    };

    if let Err(e) = res {
        eprintln!("\n[-] HyperX CLI Error: {}", e);
        std::process::exit(1);
    }
}
