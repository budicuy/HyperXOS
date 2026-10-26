use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    /// Validate host build dependencies, KVM hardware acceleration, and configurations
    Check,
    /// Build or synchronize local pacman repository database
    Repo {
        #[arg(default_value = "update")]
        action: String,
    },
    /// Build HyperXOS ISO (defaults to fast development mode, use --prod for release)
    Build {
        /// Build in production mode (ultra compression, clean release)
        #[arg(long, short)]
        prod: bool,
    },
    /// Run HyperXOS in QEMU virtual machine test harness (dev, install, run)
    Vm {
        /// Mode to run: 'dev' (live ISO), 'install' (clean 40G disk), or 'run' (boot installed OS)
        #[arg(default_value = "dev")]
        mode: String,
    },
    /// Clean temporary build directories and artifacts
    Clean {
        /// Also remove output ISOs and virtual test disk images
        #[arg(long, short)]
        all: bool,
    },
}

fn get_project_root() -> PathBuf {
    if let Ok(cur) = std::env::current_dir() {
        if cur.join("archiso/profiledef.sh").exists() {
            return cur;
        }
        if cur.join("../../archiso/profiledef.sh").exists() {
            if let Ok(p) = cur.join("../..").canonicalize() {
                return p;
            }
        }
    }
    PathBuf::from("/home/budicuy/Dokumen/HyperXOS")
}

fn cmd_check(root: &Path) -> Result<(), String> {
    println!("==> [HyperXOS] Validating host environment and project configurations...");

    let tools = [
        ("mkarchiso", "archiso"),
        ("qemu-system-x86_64", "qemu-desktop"),
        ("xorriso", "xorriso"),
    ];

    for (tool, pkg) in tools {
        if Command::new("which").arg(tool).output().map(|o| o.status.success()).unwrap_or(false) {
            println!("[✓] Host dependency '{}' is installed.", tool);
        } else {
            return Err(format!("Missing required dependency '{}'. Install via: sudo pacman -S {}", tool, pkg));
        }
    }

    // Check KVM acceleration
    let kvm_path = Path::new("/dev/kvm");
    if kvm_path.exists() {
        if let Ok(meta) = fs::metadata(kvm_path) {
            use std::os::unix::fs::PermissionsExt;
            let mode = meta.permissions().mode();
            if mode & 0o006 != 0 || mode & 0o060 != 0 {
                println!("[✓] Hardware virtualization acceleration (/dev/kvm) is accessible.");
            } else {
                println!("[!] Warning: /dev/kvm exists but check group permissions.");
            }
        }
    } else {
        println!("[!] Warning: /dev/kvm not found; QEMU will run in software TCG mode.");
    }

    // Check archiso files
    let profiledef = root.join("archiso/profiledef.sh");
    if profiledef.exists() {
        println!("[✓] archiso/profiledef.sh found.");
    } else {
        return Err("archiso/profiledef.sh missing!".to_string());
    }

    let packages_file = root.join("archiso/packages.x86_64");
    if packages_file.exists() {
        let count = fs::read_to_string(&packages_file)
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .count();
        println!("[✓] archiso/packages.x86_64 found ({} packages listed).", count);
    } else {
        return Err("archiso/packages.x86_64 missing!".to_string());
    }

    println!("[✓] All validations completed successfully!");
    Ok(())
}

fn cmd_repo(root: &Path, _action: &str) -> Result<(), String> {
    println!("==> [HyperXOS] Updating local pacman repository database...");
    let repo_dir = root.join("repo/x86_64");
    fs::create_dir_all(&repo_dir).map_err(|e| e.to_string())?;

    // Collect .pkg.tar.zst
    let mut pkgs = Vec::new();
    if let Ok(entries) = fs::read_dir(&repo_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".pkg.tar.zst") {
                pkgs.push(name);
            }
        }
    }

    if pkgs.is_empty() {
        println!("  [i] No packages found in repo/x86_64. Initializing empty database.");
        return Ok(());
    }

    println!("  -> Found {} package(s). Adding to hyperxos.db.tar.gz...", pkgs.len());
    let _ = fs::remove_file(repo_dir.join("hyperxos.db.tar.gz"));
    let _ = fs::remove_file(repo_dir.join("hyperxos.files.tar.gz"));

    let mut repo_add = Command::new("repo-add");
    repo_add.current_dir(&repo_dir);
    repo_add.arg("hyperxos.db.tar.gz");
    for p in &pkgs {
        repo_add.arg(p);
    }

    let status = repo_add.status().map_err(|e| format!("Failed to run repo-add: {}", e))?;
    if !status.success() {
        return Err("repo-add failed".to_string());
    }

    // Create symlinks
    let _ = std::os::unix::fs::symlink("hyperxos.db.tar.gz", repo_dir.join("hyperxos.db"));
    let _ = std::os::unix::fs::symlink("hyperxos.files.tar.gz", repo_dir.join("hyperxos.files"));

    // Mirror to archiso
    let airootfs_repo = root.join("archiso/airootfs/opt/hyperxos-repo/x86_64");
    let _ = fs::create_dir_all(&airootfs_repo);
    let mut cp = Command::new("cp");
    cp.args(["-af", &format!("{}/.", repo_dir.display()), &format!("{}/", airootfs_repo.display())]);
    let _ = cp.status();

    println!("[✓] Repository database successfully updated and mirrored to airootfs.");
    Ok(())
}

fn cmd_build(root: &Path, prod: bool) -> Result<(), String> {
    let mode = if prod { "prod" } else { "dev" };
    println!("==================================================");
    println!("  HyperXOS ISO Build System (Pure Rust)");
    println!("  Mode: {}", mode);
    println!("==================================================");

    // 1. Sync repository first
    cmd_repo(root, "update")?;

    // 2. Run build-iso script or mkarchiso
    let script = root.join("scripts/build-iso.sh");
    if script.exists() {
        let status = Command::new("bash")
            .arg(&script)
            .arg(mode)
            .status()
            .map_err(|e| format!("Failed to run build: {}", e))?;
        if !status.success() {
            return Err("Build failed".to_string());
        }
    } else {
        return Err("scripts/build-iso.sh not found".to_string());
    }

    Ok(())
}

fn cmd_vm(root: &Path, mode: &str) -> Result<(), String> {
    println!("==> [HyperXOS] Launching QEMU VM (Mode: {})...", mode);
    let script = root.join("scripts/run-qemu.sh");
    if script.exists() {
        let status = Command::new("bash")
            .arg(&script)
            .arg(mode)
            .status()
            .map_err(|e| format!("Failed to run QEMU: {}", e))?;
        if !status.success() {
            return Err("QEMU session ended with error".to_string());
        }
    } else {
        return Err("scripts/run-qemu.sh not found".to_string());
    }
    Ok(())
}

fn cmd_clean(root: &Path, all: bool) -> Result<(), String> {
    println!("==> [HyperXOS] Cleaning temporary build artifacts...");
    let script = root.join("scripts/clean.sh");
    let mut cmd = Command::new("bash");
    cmd.arg(&script);
    if all {
        cmd.arg("--all");
    }
    let status = cmd.status().map_err(|e| format!("Failed to clean: {}", e))?;
    if !status.success() {
        return Err("Cleanup failed".to_string());
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let root = get_project_root();

    let res = match cli.command {
        Commands::Check => cmd_check(&root),
        Commands::Repo { action } => cmd_repo(&root, &action),
        Commands::Build { prod } => cmd_build(&root, prod),
        Commands::Vm { mode } => cmd_vm(&root, &mode),
        Commands::Clean { all } => cmd_clean(&root, all),
    };

    if let Err(e) = res {
        eprintln!("\n[-] HyperX CLI Error: {}", e);
        std::process::exit(1);
    }
}
