use std::path::Path;
use std::process::Command;

use crate::checksum::run_checksum;
use crate::util::{command_exists, find_latest_iso};

pub fn run_release(root: &Path, checksum_only: bool) -> Result<(), String> {
    if checksum_only {
        return run_checksum(root);
    }
    println!("======================================================================");
    println!("               HyperXOS GitHub Release Manager (Pure Rust)            ");
    println!("======================================================================");

    let latest_iso = find_latest_iso(root).ok_or_else(|| {
        "No HyperXOS ISO found in out/! Please run 'hyperx build --prod' first.".to_string()
    })?;

    let iso_name = latest_iso.file_name().and_then(|n| n.to_str()).unwrap_or("");
    // Parse version from hyperxos-YYYY.MM.DD-x86_64.iso
    let version = iso_name
        .trim_start_matches("hyperxos-")
        .trim_end_matches("-x86_64.iso");
    let tag = format!("v{}", version);

    println!("[+] ISO Image Detected: {}", iso_name);
    println!("[+] Release Version:    {}", version);
    println!("[+] Target Git Tag:     {}", tag);

    // Ensure Checksums exist
    let sha_file = latest_iso.with_extension("iso.sha256");
    let b2_file = latest_iso.with_extension("iso.b2sum");

    if !sha_file.exists() || !b2_file.exists() {
        println!("[!] Checksums missing or incomplete. Generating now...");
        run_checksum(root)?;
    }

    let mut assets = vec![
        latest_iso.clone(),
        sha_file.clone(),
        b2_file.clone(),
    ];

    let pkglist = root.join(format!("out/hyperxos-{}-x86_64.pkglist", version));
    if pkglist.exists() {
        assets.push(pkglist);
    }

    // Ensure local git tag
    let check_tag = Command::new("git")
        .args(["rev-parse", &tag])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !check_tag {
        println!("[+] Creating local git tag '{}'...", tag);
        let tag_msg = format!("HyperXOS {} Production Release", version);
        let _ = Command::new("git")
            .args(["tag", "-a", &tag, "-m", &tag_msg])
            .status();
    } else {
        println!("[✓] Local git tag '{}' already exists.", tag);
    }

    // Check GitHub CLI (gh)
    if command_exists("gh") {
        let gh_auth = Command::new("gh")
            .args(["auth", "status"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if gh_auth {
            println!("[+] GitHub CLI is authenticated. Preparing release...");
            let repo_name = "budicuy/HyperXOS";
            let release_notes = root.join("RELEASE_NOTES.md");

            let release_exists = Command::new("gh")
                .args(["release", "view", &tag, "--repo", repo_name])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            if release_exists {
                println!("[!] Release '{}' already exists on GitHub. Uploading updated assets...", tag);
                let mut upload = Command::new("gh");
                upload.args(["release", "upload", &tag, "--clobber", "--repo", repo_name]);
                for a in &assets {
                    upload.arg(a);
                }
                let status = upload.status().map_err(|e| e.to_string())?;
                if !status.success() {
                    return Err("Failed to upload release assets via gh".to_string());
                }
            } else {
                println!("[+] Creating new release on GitHub...");
                let mut create = Command::new("gh");
                create.args([
                    "release", "create", &tag,
                    "--repo", repo_name,
                    "--title", &format!("HyperXOS {} — Official Release", version),
                ]);
                if release_notes.exists() {
                    create.args(["--notes-file", release_notes.to_str().unwrap()]);
                }
                for a in &assets {
                    create.arg(a);
                }
                let status = create.status().map_err(|e| e.to_string())?;
                if !status.success() {
                    return Err("Failed to create GitHub release via gh".to_string());
                }
            }

            println!("[✓] GitHub release successfully published!");
            println!("    URL: https://github.com/{}/releases/tag/{}", repo_name, tag);
            return Ok(());
        }
    }

    // Fallback instructions
    println!("----------------------------------------------------------------------");
    println!("[i] GitHub CLI (gh) is not installed or not logged in.");
    println!("    You can publish this release manually:");
    println!("    1. Push commit and tag to GitHub:");
    println!("       git push origin main");
    println!("       git push origin {}", tag);
    println!("    2. Open release page: https://github.com/budicuy/HyperXOS/releases/new?tag={}", tag);
    println!("    3. Title: HyperXOS {} — Official Release", version);
    println!("    4. Upload artifacts from out/:");
    for a in &assets {
        println!("       - {}", a.file_name().unwrap().to_string_lossy());
    }
    println!("======================================================================");
    Ok(())
}
