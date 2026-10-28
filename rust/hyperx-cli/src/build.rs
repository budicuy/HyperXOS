use std::fs;
use std::path::Path;
use std::process::Command;

use crate::checksum::run_checksum;
use crate::util::{find_latest_iso, run_stream};

pub fn run_build(root: &Path, prod: bool, repo_only: bool) -> Result<(), String> {
    let mode = if prod { "prod" } else { "dev" };
    println!("==================================================");
    println!("  HyperXOS Unified Build Engine (Pure Rust)");
    println!("  Mode: {}", mode);
    println!("  Time: {}", chrono_lite_timestamp());
    println!("==================================================");

    let out_dir = root.join("out");
    let work_dir = Path::new("/var/tmp/hyperxos-work");
    let profile_dir = root.join("archiso");
    let profiledef = profile_dir.join("profiledef.sh");
    let mkinit_conf = profile_dir.join("airootfs/etc/mkinitcpio.conf.d/archiso.conf");
    let packages_file = profile_dir.join("packages.x86_64");
    let hash_file = work_dir.join(".packages.hash");
    let max_prod_bytes: u64 = 2_000_000_000; // 2.0 GB decimal limit

    fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

    // Step 1: Build Base Packages
    println!("==> [1/4] Building base system packages...");
    crate::repo::build_base_packages(root)?;

    // Step 2: Build Rust Packages
    println!(
        "==> [2/4] Compiling & packaging HyperXOS Rust packages (mode: '{}')...",
        mode
    );
    crate::repo::build_and_sync_rust_packages(root, prod)?;

    // Step 3: Build & update pacman repository database
    println!("==> [3/4] Building local pacman repository database & mirroring to airootfs...");
    crate::repo::run_repo(root, "update")?;

    if repo_only {
        println!("==================================================");
        println!("  HyperXOS Packages & Repository Built Successfully!");
        println!("  (--repo-only specified: skipping ISO creation)");
        println!("==================================================");
        return Ok(());
    }

    // Update archiso pacman.conf server path
    let pacman_conf = profile_dir.join("pacman.conf");
    if pacman_conf.exists() {
        if let Ok(content) = fs::read_to_string(&pacman_conf) {
            let target_repo = format!("file://{}/repo/$arch", root.display());
            let mut new_lines = Vec::new();
            for line in content.lines() {
                if line.trim_start().starts_with("Server = file://") && line.contains("/repo/$arch") {
                    new_lines.push(format!("Server = {}", target_repo));
                } else {
                    new_lines.push(line.to_string());
                }
            }
            let _ = fs::write(&pacman_conf, new_lines.join("\n"));
        }
    }

    // Step 4: Build ISO and checksums
    println!("==> [4/4] Building HyperXOS ISO and generating checksums (mode: '{}')...", mode);

    if prod {
        println!("  -> Applying PRODUCTION settings: Ultra-compressed (SquashFS zstd -22 1M, Initramfs xz -9e)");
        println!("  -> Strict zero-tolerance quality policy: no errors, no conflicts, strict < 2.0GB size.");

        // SquashFS zstd -22 1M
        modify_profiledef_compression(&profiledef, "('-comp' 'zstd' '-Xcompression-level' '22' '-b' '1M' '-no-xattrs')")?;

        // Initramfs xz -9e
        let _ = fs::create_dir_all(mkinit_conf.parent().unwrap());
        let mkinit_content = "HOOKS=(base udev microcode modconf memdisk archiso archiso_loop_mnt block filesystems keyboard)\nCOMPRESSION=\"xz\"\nCOMPRESSION_OPTIONS=(-9e --threads=0)\nMODULES_DECOMPRESS=\"yes\"\n";
        let _ = fs::write(&mkinit_conf, mkinit_content);

        // Clean work directory for 100% clean build
        if work_dir.exists() {
            println!("  -> [PROD] Cleaning workspace {} for clean production compile...", work_dir.display());
            let _ = Command::new("sudo").args(["umount", "-R", "/var/tmp/hyperxos-work"]).output();
            let _ = Command::new("sudo").args(["rm", "-rf", "/var/tmp/hyperxos-work"]).output();
        }
        let _ = Command::new("sudo").args(["install", "-d", "-m", "0777", "/var/tmp/hyperxos-work"]).output();
    } else {
        println!("  -> Applying DEVELOPMENT settings: Blazing fast build with build caching");
        println!("  -> Fast compression: SquashFS zstd level 1 (fast pack), Initramfs zstd level 1");

        // SquashFS zstd -1 256K
        modify_profiledef_compression(&profiledef, "('-comp' 'zstd' '-Xcompression-level' '1' '-b' '256K' '-no-xattrs')")?;

        // Initramfs zstd -1
        let _ = fs::create_dir_all(mkinit_conf.parent().unwrap());
        let mkinit_content = "HOOKS=(base udev microcode modconf memdisk archiso archiso_loop_mnt block filesystems keyboard)\nCOMPRESSION=\"zstd\"\nCOMPRESSION_OPTIONS=(-1 -T0)\nMODULES_DECOMPRESS=\"no\"\n";
        let _ = fs::write(&mkinit_conf, mkinit_content);

        // Caching check (based on archiso/packages.x86_64 list so Rust binary updates still hit the fast cache!)
        let current_pkg_list_hash = compute_package_list_hash(&packages_file);
        let cached_hash = fs::read_to_string(&hash_file).unwrap_or_default().trim().to_string();
        let airootfs_work = work_dir.join("x86_64/airootfs");
        let packages_marker = work_dir.join("base._make_packages");

        if airootfs_work.exists() && packages_marker.exists() && cached_hash == current_pkg_list_hash {
            println!("  -> [CACHE HIT] Reusing cached pacstrap airootfs for instant build...");
            println!("  -> Syncing overlay archiso/airootfs changes & newly built dev binaries...");
            let mut cp_cmd = Command::new("sudo");
            cp_cmd.args(["cp", "-af", "--no-preserve=ownership,mode", &format!("{}/.", profile_dir.join("airootfs").display()), &format!("{}/", airootfs_work.display())]);
            let _ = cp_cmd.status();

            // Inject freshly built dev binaries directly into cached airootfs/usr/bin/
            let debug_dir = root.join("rust/target/debug");
            let usr_bin = airootfs_work.join("usr/bin");
            for bin_name in ["hyper-x-tools", "hyper-x-installer", "hyper-x-hw"] {
                let src_bin = debug_dir.join(bin_name);
                let dst_bin = usr_bin.join(bin_name);
                if src_bin.exists() {
                    let _ = Command::new("sudo")
                        .args([
                            "install",
                            "-Dm755",
                            src_bin.to_str().unwrap(),
                            dst_bin.to_str().unwrap(),
                        ])
                        .status();
                }
            }

            // Invalidate downstream packaging markers
            let invalidate = [
                "base._mkairootfs_squashfs",
                "base._prepare_airootfs_image",
                "base._make_iso_image",
                "iso._build_iso_image",
                "build._build_buildmode_iso",
            ];
            for inv in invalidate {
                let p = work_dir.join(inv);
                if p.exists() {
                    let _ = Command::new("sudo").args(["rm", "-f", p.to_str().unwrap()]).status();
                }
            }
            let _ = Command::new("sudo").args(["rm", "-f", "/var/tmp/hyperxos-work/iso/hyperxos/x86_64/airootfs.sfs"]).status();
        } else {
            println!("  -> [CACHE MISS] Initializing clean workspace (package changes or first build)...");
            if work_dir.exists() {
                let _ = Command::new("sudo").args(["umount", "-R", "/var/tmp/hyperxos-work"]).output();
                let _ = Command::new("sudo").args(["rm", "-rf", "/var/tmp/hyperxos-work"]).output();
            }
            let _ = Command::new("sudo").args(["install", "-d", "-m", "0777", "/var/tmp/hyperxos-work"]).output();
        }
    }

    // Run mkarchiso
    println!("  -> Running mkarchiso...");
    println!("     Workdir: {}", work_dir.display());
    println!("     Output:  {}", out_dir.display());
    println!("     Profile: {}", profile_dir.display());

    let mut mkarchiso_cmd = Command::new("sudo");
    mkarchiso_cmd.args([
        "mkarchiso",
        "-v",
        "-w", work_dir.to_str().unwrap(),
        "-o", out_dir.to_str().unwrap(),
        profile_dir.to_str().unwrap(),
    ]);

    run_stream(&mut mkarchiso_cmd, "mkarchiso")?;

    // Save package list hash on successful dev build
    if !prod {
        let current_pkg_list_hash = compute_package_list_hash(&packages_file);
        let _ = fs::write(&hash_file, &current_pkg_list_hash);
    }

    // Fix ownership of built artifacts
    println!("  -> Setting ownership of built artifacts in {}...", out_dir.display());
    let uid = users_id();
    let mut chown_cmd = Command::new("sudo");
    chown_cmd.args(["chown", "-R", &uid, out_dir.to_str().unwrap()]);
    let _ = chown_cmd.status();

    // Verification and Reporting
    let latest_iso = find_latest_iso(root).ok_or_else(|| {
        "Expected ISO was not created in out/!".to_string()
    })?;

    let meta = fs::metadata(&latest_iso).map_err(|e| e.to_string())?;
    let size_bytes = meta.len();
    let size_mb = size_bytes / 1024 / 1024;

    println!("==================================================");
    println!("  HyperXOS ISO Build Completed Successfully!");
    println!("  Mode:      {}", mode);
    println!("  Artifact:  {}", latest_iso.display());
    println!("  File Size: {} MB ({} bytes)", size_mb, size_bytes);

    if prod {
        if size_bytes >= max_prod_bytes {
            return Err(format!(
                "Production ISO size ({} bytes / {} MB) EXCEEDS 2.0 GB limit ({} bytes)!",
                size_bytes, size_mb, max_prod_bytes
            ));
        }
        println!("[✓] Production size check PASSED: strictly under 2.0 GB ({} < {}).", size_bytes, max_prod_bytes);
    }

    println!("==> Generating ISO checksums...");
    let _ = run_checksum(root);

    println!("==================================================");
    Ok(())
}

fn compute_package_list_hash(pkg_file: &Path) -> String {
    let mut hasher = String::new();
    if let Ok(c1) = fs::read_to_string(pkg_file) {
        hasher.push_str(&c1);
    }
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    hasher.hash(&mut h);
    format!("{:x}", h.finish())
}

fn modify_profiledef_compression(profiledef: &Path, options: &str) -> Result<(), String> {
    if !profiledef.exists() {
        return Err("profiledef.sh not found".to_string());
    }
    let content = fs::read_to_string(profiledef).map_err(|e| e.to_string())?;
    let mut new_lines = Vec::new();
    for line in content.lines() {
        if line.starts_with("airootfs_image_tool_options=") {
            new_lines.push(format!("airootfs_image_tool_options={}", options));
        } else {
            new_lines.push(line.to_string());
        }
    }
    fs::write(profiledef, new_lines.join("\n")).map_err(|e| e.to_string())
}

fn users_id() -> String {
    let uid = Command::new("id").arg("-u").output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_else(|_| "1000".to_string());
    let gid = Command::new("id").arg("-g").output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_else(|_| "1000".to_string());
    format!("{}:{}", uid, gid)
}

fn chrono_lite_timestamp() -> String {
    Command::new("date")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown Date".to_string())
}
