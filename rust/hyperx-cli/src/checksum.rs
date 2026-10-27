use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run_checksum(root: &Path) -> Result<(), String> {
    println!("==> [HyperXOS] Computing official checksums (SHA256 & B2SUM)...");
    let out_dir = root.join("out");
    if !out_dir.exists() {
        return Err("Directory 'out/' does not exist. Build the ISO first!".to_string());
    }

    let mut found = 0;
    if let Ok(entries) = fs::read_dir(&out_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("hyperxos-") && name.ends_with(".iso") {
                    found += 1;
                    println!("  -> Processing: {}", name);

                    // 1. SHA256
                    let sha_output = Command::new("sha256sum")
                        .arg(name)
                        .current_dir(&out_dir)
                        .output()
                        .map_err(|e| format!("Failed to run sha256sum: {}", e))?;

                    if sha_output.status.success() {
                        let sha_file = out_dir.join(format!("{}.sha256", name));
                        fs::write(&sha_file, &sha_output.stdout).map_err(|e| e.to_string())?;
                        let sha_str = String::from_utf8_lossy(&sha_output.stdout);
                        let hash = sha_str.split_whitespace().next().unwrap_or("");
                        println!("     SHA256: {}", hash);
                    }

                    // 2. B2SUM
                    let b2_output = Command::new("b2sum")
                        .arg(name)
                        .current_dir(&out_dir)
                        .output()
                        .map_err(|e| format!("Failed to run b2sum: {}", e))?;

                    if b2_output.status.success() {
                        let b2_file = out_dir.join(format!("{}.b2sum", name));
                        fs::write(&b2_file, &b2_output.stdout).map_err(|e| e.to_string())?;
                        let b2_str = String::from_utf8_lossy(&b2_output.stdout);
                        let hash = b2_str.split_whitespace().next().unwrap_or("");
                        println!("     B2SUM:  {}", hash);
                    }
                }
            }
        }
    }

    if found == 0 {
        return Err("No HyperXOS ISO found in out/ to checksum.".to_string());
    }

    println!("[✓] Checksums generation completed successfully ({} ISO(s) processed).", found);
    Ok(())
}
