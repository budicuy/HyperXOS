use std::fs;
use std::path::Path;
use std::process::Command;

use crate::util::run_stream;

pub const OFFICIAL_PACKAGES: &[&str] = &[
    "hyperx-de-1.0.0-1-x86_64.pkg.tar.zst",
    "hyperx-file-manager-1.0.0-1-x86_64.pkg.tar.zst",
    "hyper-x-hw-1.0.0-1-x86_64.pkg.tar.zst",
    "hyper-x-installer-1.0.0-1-x86_64.pkg.tar.zst",
    "hyper-x-tools-2.2.0-1-x86_64.pkg.tar.zst",
    "hyperxos-branding-1.0.0-3-any.pkg.tar.zst",
    "hyperxos-default-settings-1.0.0-4-any.pkg.tar.zst",
    "hyperxos-hooks-1.0.0-1-any.pkg.tar.zst",
    "hyperxos-keyring-1.0.0-1-any.pkg.tar.zst",
    "hyperxos-mirrorlist-1.0.0-2-any.pkg.tar.zst",
];

pub const RELEASE_BASE_URL: &str = "https://github.com/budicuy/HyperXOS/releases/latest/download";

/// Ensure all 8 official HyperXOS packages exist locally in repo/x86_64/
pub fn ensure_packages(root: &Path) -> Result<(), String> {
    let repo_dir = root.join("repo/x86_64");
    fs::create_dir_all(&repo_dir).map_err(|e| e.to_string())?;

    let mut missing = Vec::new();
    for pkg in OFFICIAL_PACKAGES {
        let pkg_path = repo_dir.join(pkg);
        if !pkg_path.exists() {
            missing.push(*pkg);
        }
    }

    if !missing.is_empty() {
        println!("==> [HyperXOS] Fetching {} official package(s) from GitHub Releases...", missing.len());
        for pkg in &missing {
            let pkg_path = repo_dir.join(pkg);
            let url = format!("{}/{}", RELEASE_BASE_URL, pkg);
            println!("  -> Downloading {} ...", pkg);
            let mut curl = Command::new("curl");
            curl.args([
                "-L",
                "--fail",
                "-o",
                pkg_path.to_str().unwrap(),
                &url,
            ]);
            let s = curl.status();
            if s.is_err() || !s.unwrap().success() {
                let _ = fs::remove_file(&pkg_path);
                println!("  [!] Warning: Could not download {} from GitHub Releases mirror (will try local fallback).", pkg);
            }
        }
    }

    // Ensure database is updated
    run_repo(root, "update")
}

pub fn run_repo(root: &Path, _action: &str) -> Result<(), String> {
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
        let mut tar_db = Command::new("tar");
        tar_db.current_dir(&repo_dir);
        tar_db.args(["-czf", "hyperxos.db.tar.gz", "-T", "/dev/null"]);
        let _ = tar_db.status();

        let mut tar_files = Command::new("tar");
        tar_files.current_dir(&repo_dir);
        tar_files.args(["-czf", "hyperxos.files.tar.gz", "-T", "/dev/null"]);
        let _ = tar_files.status();

        let _ = std::os::unix::fs::symlink("hyperxos.db.tar.gz", repo_dir.join("hyperxos.db"));
        let _ = std::os::unix::fs::symlink("hyperxos.files.tar.gz", repo_dir.join("hyperxos.files"));
        return Ok(());
    }

    println!("  -> Found {} package(s). Adding to hyperxos.db.tar.gz...", pkgs.len());
    let _ = fs::remove_file(repo_dir.join("hyperxos.db"));
    let _ = fs::remove_file(repo_dir.join("hyperxos.db.tar.gz"));
    let _ = fs::remove_file(repo_dir.join("hyperxos.files"));
    let _ = fs::remove_file(repo_dir.join("hyperxos.files.tar.gz"));

    let mut repo_add = Command::new("repo-add");
    repo_add.current_dir(&repo_dir);
    repo_add.arg("hyperxos.db.tar.gz");
    for p in &pkgs {
        repo_add.arg(p);
    }

    run_stream(&mut repo_add, "repo-add")?;

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
