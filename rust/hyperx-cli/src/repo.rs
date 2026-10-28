use std::fs;
use std::path::Path;
use std::process::Command;

use crate::util::run_stream;

pub const OFFICIAL_PACKAGES: &[&str] = &[
    "hyperx-de-1.0.0-1-x86_64.pkg.tar.zst",
    "hyper-x-hw-1.0.0-1-x86_64.pkg.tar.zst",
    "hyper-x-installer-1.0.0-1-x86_64.pkg.tar.zst",
    "hyper-x-tools-2.2.0-1-x86_64.pkg.tar.zst",
    "hyperxos-branding-1.0.0-3-any.pkg.tar.zst",
    "hyperxos-default-settings-1.0.0-4-any.pkg.tar.zst",
    "hyperxos-hooks-1.0.0-1-any.pkg.tar.zst",
    "hyperxos-keyring-1.0.0-1-any.pkg.tar.zst",
    "hyperxos-mirrorlist-1.0.0-2-any.pkg.tar.zst",
];

pub const RUST_PKG_DIRS: &[&str] = &[
    "hyperx-de",
    "hyper-x-tools",
    "hyper-x-installer",
    "hyper-x-hw",
];

pub const RELEASE_BASE_URL: &str = "https://github.com/budicuy/HyperXOS/releases/latest/download";

/// Compiles all 4 HyperXOS Rust packages in either Dev (`prod = false`) or Release (`prod = true`) mode,
/// packages them via `makepkg -fd`, copies the `.pkg.tar.zst` archives into `repo/x86_64/`, and updates the repo DB.
pub fn build_and_sync_rust_packages(root: &Path, prod: bool) -> Result<(), String> {
    let build_mode = if prod { "release" } else { "dev" };
    let profile_label = if prod {
        "PRODUCTION (--release: opt-level=3, thin-LTO, strip=symbols)"
    } else {
        "DEVELOPMENT (dev: opt-level=0, debug=true, strip=none, incremental=true)"
    };

    println!(
        "==> [HyperXOS] Compiling Rust packages in {} mode...",
        profile_label
    );

    let manifest = root.join("rust/Cargo.toml");
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("build");
    if prod {
        cargo_cmd.arg("--release");
    }
    cargo_cmd.args([
        "--manifest-path",
        manifest.to_str().unwrap(),
        "--locked",
        "--bin",
        "hyperx-de",
        "--bin",
        "hyper-x-tools",
        "--bin",
        "hyper-x-installer",
        "--bin",
        "hyper-x-hw",
    ]);
    run_stream(&mut cargo_cmd, "cargo build workspace packages")?;

    // Guard: production packages must never contain unstripped debug binaries.
    if prod {
        for bin in ["hyperx-de", "hyper-x-tools", "hyper-x-installer", "hyper-x-hw"] {
            let path = root.join("rust/target/release").join(bin);
            let info = Command::new("file")
                .arg(&path)
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .unwrap_or_default();
            if info.contains("with debug_info") || info.contains("not stripped") {
                return Err(format!(
                    "Production guard: {} is not stripped (debug build leaked into release).",
                    path.display()
                ));
            }
        }
        println!("  [✓] Production guard: all 4 release binaries are stripped & optimized.");
    }

    let repo_dir = root.join("repo/x86_64");
    fs::create_dir_all(&repo_dir).map_err(|e| e.to_string())?;

    for pkg_name in RUST_PKG_DIRS {
        let pkg_dir = root.join("packages").join(pkg_name);
        if !pkg_dir.exists() {
            continue;
        }
        println!(
            "  -> Packaging {} (HYPERX_BUILD_MODE={})...",
            pkg_name, build_mode
        );

        let mut makepkg = Command::new("makepkg");
        makepkg
            .current_dir(&pkg_dir)
            .env("HYPERX_BUILD_MODE", build_mode)
            .args(["-f", "-d", "--noconfirm"]);
        run_stream(&mut makepkg, &format!("makepkg {}", pkg_name))?;

        // Move generated .pkg.tar.zst to repo/x86_64/ and clean makepkg pkg/ & src/ dirs
        let _ = fs::remove_dir_all(pkg_dir.join("pkg"));
        let _ = fs::remove_dir_all(pkg_dir.join("src"));
        if let Ok(entries) = fs::read_dir(&pkg_dir) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                if fname.starts_with(pkg_name) && fname.ends_with(".pkg.tar.zst") {
                    let dest = repo_dir.join(&fname);
                    fs::copy(entry.path(), &dest).map_err(|e| e.to_string())?;
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }

    ensure_packages(root)
}

/// Ensure all 9 official HyperXOS packages exist locally in repo/x86_64/
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
        println!(
            "==> [HyperXOS] Fetching {} missing official package(s) from GitHub Releases...",
            missing.len()
        );
        for pkg in &missing {
            let pkg_path = repo_dir.join(pkg);
            let url = format!("{}/{}", RELEASE_BASE_URL, pkg);
            println!("  -> Downloading {} ...", pkg);
            let mut curl = Command::new("curl");
            curl.args(["-L", "--fail", "-o", pkg_path.to_str().unwrap(), &url]);
            let s = curl.status();
            if s.is_err() || !s.unwrap().success() {
                let _ = fs::remove_file(&pkg_path);
                println!(
                    "  [!] Warning: Could not download {} from GitHub Releases mirror (will try local fallback).",
                    pkg
                );
            }
        }
    }

    // Ensure database is updated
    run_repo(root, "update")
}

pub fn run_repo(root: &Path, action: &str) -> Result<(), String> {
    if action == "build-dev" {
        return build_and_sync_rust_packages(root, false);
    } else if action == "build-prod" {
        return build_and_sync_rust_packages(root, true);
    }

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

    println!(
        "  -> Found {} package(s). Adding to hyperxos.db.tar.gz...",
        pkgs.len()
    );
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
    cp.args([
        "-af",
        &format!("{}/.", repo_dir.display()),
        &format!("{}/", airootfs_repo.display()),
    ]);
    let _ = cp.status();

    println!("[✓] Repository database successfully updated and mirrored to airootfs.");
    Ok(())
}
