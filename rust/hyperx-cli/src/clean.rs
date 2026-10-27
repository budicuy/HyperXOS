use std::fs;
use std::path::Path;
use std::process::Command;


pub fn run_clean(root: &Path, all: bool) -> Result<(), String> {
    println!("==> [HyperXOS] Cleaning temporary build artifacts...");

    // 1. Remove local work directory
    let local_work = root.join("work");
    if local_work.exists() {
        println!("  -> Removing {}...", local_work.display());
        let mut rm_cmd = Command::new("sudo");
        rm_cmd.args(["rm", "-rf", local_work.to_str().unwrap()]);
        let _ = rm_cmd.status();
    }

    // 2. Unmount and remove /var/tmp/hyperxos-work
    let var_work = Path::new("/var/tmp/hyperxos-work");
    if var_work.exists() {
        println!("  -> Unmounting and removing /var/tmp/hyperxos-work...");
        let mut umount_cmd = Command::new("sudo");
        umount_cmd.args(["umount", "-R", "/var/tmp/hyperxos-work"]);
        let _ = umount_cmd.output();

        let mut rm_var = Command::new("sudo");
        rm_var.args(["rm", "-rf", "/var/tmp/hyperxos-work"]);
        let _ = rm_var.status();
    }

    // 3. Remove /tmp/hyperxos-tmp if present
    let tmp_work = Path::new("/tmp/hyperxos-tmp");
    if tmp_work.exists() {
        println!("  -> Removing /tmp/hyperxos-tmp...");
        let mut rm_tmp = Command::new("sudo");
        rm_tmp.args(["rm", "-rf", "/tmp/hyperxos-tmp"]);
        let _ = rm_tmp.status();
    }

    // 4. If all requested, clean output and virtual disks
    if all {
        println!("  -> Cleaning out/ directory and virtual test disks...");
        let out_dir = root.join("out");
        if out_dir.exists() {
            let _ = fs::remove_dir_all(&out_dir);
        }

        let extra_files = [
            root.join("rust/hyperx-vm/hyperxos-install.qcow2"),
            root.join("rust/hyperx-vm/OVMF_VARS.4m.fd"),
            root.join("hyperxos-install.qcow2"),
            root.join("hyperxos-test.qcow2"),
            root.join("OVMF_VARS.4m.fd"),
        ];

        for f in extra_files {
            if f.exists() {
                let _ = fs::remove_file(&f);
            }
        }
    }

    println!("[✓] Cleanup completed successfully.");
    Ok(())
}
