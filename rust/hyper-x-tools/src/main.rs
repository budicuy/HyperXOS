slint::include_modules!();

mod core;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use slint::{ComponentHandle, ModelRc, VecModel};

use crate::core::kernel_manager::KernelManager;
use crate::core::maintenance::Maintenance;
use crate::core::package_manager::PackageManager;
use crate::core::snapshot_manager::SnapshotManager;
use crate::core::system_monitor::SystemMonitor;
use crate::core::system_tweaker::SystemTweaker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;

    // 1. Initial State Population
    refresh_kernel_page(&app);
    refresh_gaming_page(&app);
    refresh_snapshot_page(&app);
    refresh_store_page(&app, "Semua", "");

    // 2. Hardware Monitoring Background Worker
    let monitor_weak = app.as_weak();
    let monitor_arc = Arc::new(Mutex::new(SystemMonitor::new()));

    // Run initial collection immediately
    {
        let mut mon = monitor_arc.lock().unwrap();
        let m = mon.collect_metrics();
        apply_metrics_to_ui(&app, &m);
    }

    // Spawn 1000ms polling thread
    let mon_thread_arc = Arc::clone(&monitor_arc);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        let metrics = {
            let mut mon = mon_thread_arc.lock().unwrap();
            mon.collect_metrics()
        };

        let weak_clone = monitor_weak.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = weak_clone.upgrade() {
                apply_metrics_to_ui(&app, &metrics);
            }
        });
    });

    // 3. UI Callbacks Registration

    // Refresh metrics on demand
    let refresh_weak = app.as_weak();
    let refresh_mon = Arc::clone(&monitor_arc);
    app.on_refresh_metrics(move || {
        let metrics = {
            let mut mon = refresh_mon.lock().unwrap();
            mon.collect_metrics()
        };
        if let Some(app) = refresh_weak.upgrade() {
            apply_metrics_to_ui(&app, &metrics);
        }
    });

    // Page Switching
    let switch_weak = app.as_weak();
    app.on_switch_page(move |page_idx| {
        if let Some(app) = switch_weak.upgrade() {
            match page_idx {
                1 => refresh_kernel_page(&app),
                2 => refresh_gaming_page(&app),
                3 => refresh_snapshot_page(&app),
                4 => {} // Maintenance
                5 => refresh_store_page(&app, &app.get_active_category(), ""),
                _ => {}
            }
        }
    });

    // Clear Terminal Log
    let clear_weak = app.as_weak();
    app.on_clear_log(move || {
        if let Some(app) = clear_weak.upgrade() {
            app.set_terminal_log("Terminal dibersihkan.\n".into());
        }
    });

    // Helper macro/function for terminal logging
    let make_logger = {
        let app_weak = app.as_weak();
        move || {
            let weak = app_weak.clone();
            move |text: String| {
                let weak_clone = weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = weak_clone.upgrade() {
                        let mut cur = app.get_terminal_log().to_string();
                        cur.push_str(&text);
                        app.set_terminal_log(cur.into());
                    }
                });
            }
        }
    };

    // --- KERNEL MANAGER CALLBACKS ---
    let k_install_weak = app.as_weak();
    let k_install_logger = make_logger();
    app.on_install_kernel(move |pkg_name| {
        let pkg = pkg_name.to_string();
        let log = k_install_logger.clone();
        let weak = k_install_weak.clone();
        thread::spawn(move || {
            let headers = format!("{}-headers", pkg);
            let _ = KernelManager::execute_action(
                "pkexec",
                &["pacman", "-S", "--needed", "--noconfirm", &pkg, &headers],
                log,
            );
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_kernel_page(&app);
                }
            });
        });
    });

    let k_remove_weak = app.as_weak();
    let k_remove_logger = make_logger();
    app.on_remove_kernel(move |pkg_name| {
        let pkg = pkg_name.to_string();
        let log = k_remove_logger.clone();
        let weak = k_remove_weak.clone();
        thread::spawn(move || {
            let headers = format!("{}-headers", pkg);
            let _ = KernelManager::execute_action(
                "pkexec",
                &["pacman", "-Rns", "--noconfirm", &pkg, &headers],
                log,
            );
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_kernel_page(&app);
                }
            });
        });
    });

    let k_rebuild_logger = make_logger();
    app.on_rebuild_initramfs(move || {
        let log = k_rebuild_logger.clone();
        thread::spawn(move || {
            let _ = KernelManager::execute_action("pkexec", &["mkinitcpio", "-P"], log);
        });
    });

    // --- GAMING TWEAKS CALLBACKS ---
    let gov_weak = app.as_weak();
    let gov_logger = make_logger();
    app.on_set_governor(move |gov| {
        let gov_str = gov.to_string();
        let log = gov_logger.clone();
        let weak = gov_weak.clone();
        thread::spawn(move || {
            SystemTweaker::set_governor(&gov_str, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_gaming_page(&app);
                }
            });
        });
    });

    let map_weak = app.as_weak();
    let map_logger = make_logger();
    app.on_toggle_max_map_count(move || {
        let log = map_logger.clone();
        let weak = map_weak.clone();
        thread::spawn(move || {
            SystemTweaker::optimize_max_map_count(log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_gaming_page(&app);
                }
            });
        });
    });

    let split_weak = app.as_weak();
    let split_logger = make_logger();
    app.on_toggle_split_lock(move || {
        let current_disabled = SystemTweaker::is_split_lock_disabled();
        let log = split_logger.clone();
        let weak = split_weak.clone();
        thread::spawn(move || {
            SystemTweaker::toggle_split_lock(!current_disabled, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_gaming_page(&app);
                }
            });
        });
    });

    let gm_weak = app.as_weak();
    let gm_logger = make_logger();
    app.on_toggle_gamemode(move || {
        let is_active = SystemTweaker::is_gamemode_active();
        let log = gm_logger.clone();
        let weak = gm_weak.clone();
        thread::spawn(move || {
            SystemTweaker::toggle_gamemode(!is_active, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_gaming_page(&app);
                }
            });
        });
    });

    let gr_logger = make_logger();
    app.on_install_gaming_runtimes(move || {
        let log = gr_logger.clone();
        thread::spawn(move || {
            SystemTweaker::install_gaming_runtimes(log);
        });
    });

    // --- SNAPSHOT CALLBACKS ---
    let snap_create_weak = app.as_weak();
    let snap_create_logger = make_logger();
    app.on_create_snapshot(move |desc| {
        let d = desc.to_string();
        let log = snap_create_logger.clone();
        let weak = snap_create_weak.clone();
        thread::spawn(move || {
            SnapshotManager::create_snapshot(&d, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_snapshot_page(&app);
                }
            });
        });
    });

    let snap_restore_weak = app.as_weak();
    let snap_restore_logger = make_logger();
    app.on_restore_snapshot(move |name| {
        let n = name.to_string();
        let log = snap_restore_logger.clone();
        let weak = snap_restore_weak.clone();
        thread::spawn(move || {
            SnapshotManager::restore_snapshot(&n, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_snapshot_page(&app);
                }
            });
        });
    });

    let snap_delete_weak = app.as_weak();
    let snap_delete_logger = make_logger();
    app.on_delete_snapshot(move |name| {
        let n = name.to_string();
        let log = snap_delete_logger.clone();
        let weak = snap_delete_weak.clone();
        thread::spawn(move || {
            SnapshotManager::delete_snapshot(&n, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_snapshot_page(&app);
                }
            });
        });
    });

    // --- MAINTENANCE CALLBACKS ---
    let m_cache_logger = make_logger();
    app.on_clean_cache(move || {
        let log = m_cache_logger.clone();
        thread::spawn(move || {
            Maintenance::clean_cache(log);
        });
    });

    let m_journal_logger = make_logger();
    app.on_clean_journal(move || {
        let log = m_journal_logger.clone();
        thread::spawn(move || {
            Maintenance::clean_journal(log);
        });
    });

    let m_orphan_logger = make_logger();
    app.on_remove_orphans(move || {
        let log = m_orphan_logger.clone();
        thread::spawn(move || {
            Maintenance::remove_orphans(log);
        });
    });

    let m_rank_logger = make_logger();
    app.on_rank_mirrors(move || {
        let log = m_rank_logger.clone();
        thread::spawn(move || {
            Maintenance::rank_mirrors(log);
        });
    });

    let m_fix_logger = make_logger();
    app.on_fix_keyrings(move || {
        let log = m_fix_logger.clone();
        thread::spawn(move || {
            Maintenance::fix_keyrings(log);
        });
    });

    // --- SOFTWARE STORE CALLBACKS ---
    let cat_weak = app.as_weak();
    app.on_set_store_category(move |cat| {
        if let Some(app) = cat_weak.upgrade() {
            app.set_active_category(cat.clone());
            refresh_store_page(&app, &cat, "");
        }
    });

    let search_weak = app.as_weak();
    app.on_search_store(move |query| {
        if let Some(app) = search_weak.upgrade() {
            refresh_store_page(&app, &app.get_active_category(), &query);
        }
    });

    let pkg_install_weak = app.as_weak();
    let pkg_install_logger = make_logger();
    app.on_install_package(move |pkg_name| {
        let pkg = pkg_name.to_string();
        let log = pkg_install_logger.clone();
        let weak = pkg_install_weak.clone();
        thread::spawn(move || {
            PackageManager::install_package(&pkg, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_store_page(&app, &app.get_active_category(), "");
                }
            });
        });
    });

    let pkg_remove_weak = app.as_weak();
    let pkg_remove_logger = make_logger();
    app.on_remove_package(move |pkg_name| {
        let pkg = pkg_name.to_string();
        let log = pkg_remove_logger.clone();
        let weak = pkg_remove_weak.clone();
        thread::spawn(move || {
            PackageManager::remove_package(&pkg, log);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = weak.upgrade() {
                    refresh_store_page(&app, &app.get_active_category(), "");
                }
            });
        });
    });

    // 4. Run Application Event Loop
    app.run()?;
    Ok(())
}

fn apply_metrics_to_ui(app: &AppWindow, m: &core::system_monitor::SystemMetrics) {
    // Top Bar Quick Stats
    app.set_quick_cpu(format!("CPU: {:.1}%", m.cpu_usage_percent).into());
    app.set_quick_ram(format!("RAM: {:.1}%", m.ram_usage_percent).into());
    app.set_quick_kernel(format!("Kernel: {}", m.kernel_version).into());

    // CPU Card
    app.set_cpu_usage_text(format!("{:.1}%", m.cpu_usage_percent).into());
    app.set_cpu_progress(m.cpu_usage_percent / 100.0);
    app.set_cpu_subtext(
        format!(
            "{} Cores • {}",
            m.cpu_cores,
            m.cpu_model.chars().take(28).collect::<String>()
        )
        .into(),
    );
    if m.cpu_temp_celsius > 0 {
        app.set_cpu_extratext(format!("Suhu: {}°C", m.cpu_temp_celsius).into());
    } else {
        app.set_cpu_extratext("".into());
    }

    // RAM Card
    let ram_used_gb = m.ram_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_total_gb = m.ram_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    app.set_ram_usage_text(format!("{:.1}%", m.ram_usage_percent).into());
    app.set_ram_progress(m.ram_usage_percent / 100.0);
    app.set_ram_subtext(format!("Terpakai: {:.1} GB / {:.1} GB", ram_used_gb, ram_total_gb).into());
    if m.swap_total_bytes > 0 {
        let swap_used_gb = m.swap_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        app.set_ram_extratext(format!("Swap: {:.1} GB", swap_used_gb).into());
    } else {
        app.set_ram_extratext("".into());
    }

    // Disk Card
    let disk_used_gb = m.disk_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let disk_total_gb = m.disk_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    app.set_disk_usage_text(format!("{:.1}%", m.disk_usage_percent).into());
    app.set_disk_progress(m.disk_usage_percent / 100.0);
    app.set_disk_subtext(format!("Terpakai: {:.1} GB / {:.1} GB", disk_used_gb, disk_total_gb).into());
    app.set_disk_extratext(format!("FS: {}", m.disk_fs_type).into());

    // GPU Card
    app.set_gpu_usage_text(format!("{:.1}%", m.gpu_usage_percent).into());
    app.set_gpu_progress(m.gpu_usage_percent / 100.0);
    app.set_gpu_subtext(m.gpu_model.chars().take(32).collect::<String>().into());
    if m.vram_total_bytes > 0 {
        let vram_used_mb = m.vram_used_bytes / (1024 * 1024);
        let vram_total_mb = m.vram_total_bytes / (1024 * 1024);
        app.set_gpu_extratext(format!("VRAM: {} / {} MB", vram_used_mb, vram_total_mb).into());
    } else {
        app.set_gpu_extratext("".into());
    }

    // System Info Card
    app.set_os_name(m.os_name.clone().into());
    app.set_kernel_ver(m.kernel_version.clone().into());
    app.set_uptime_str(m.uptime_string.clone().into());
    app.set_session_type(m.display_session.clone().into());
}

fn refresh_kernel_page(app: &AppWindow) {
    let (name, ver) = KernelManager::get_running_kernel_info();
    app.set_running_kernel_name(name.into());
    app.set_running_kernel_ver(ver.into());

    let catalog = KernelManager::get_catalog();
    let slint_items: Vec<KernelItem> = catalog
        .into_iter()
        .map(|k| KernelItem {
            name: k.name.into(),
            package_name: k.package_name.into(),
            description: k.description.into(),
            installed: k.installed,
            is_running: k.is_running,
            installed_version: k.installed_version.into(),
        })
        .collect();

    app.set_kernels(ModelRc::new(VecModel::from(slint_items)));
}

fn refresh_gaming_page(app: &AppWindow) {
    let gov = SystemTweaker::get_current_governor();
    app.set_current_governor(gov.into());

    let (val, is_opt) = SystemTweaker::get_max_map_count();
    app.set_max_map_count_value(val.into());
    app.set_is_max_map_optimized(is_opt);

    let is_split_disabled = SystemTweaker::is_split_lock_disabled();
    app.set_is_split_lock_disabled(is_split_disabled);

    let is_gm_active = SystemTweaker::is_gamemode_active();
    app.set_is_gamemode_active(is_gm_active);
}

fn refresh_snapshot_page(app: &AppWindow) {
    let is_btrfs = SnapshotManager::is_btrfs_filesystem();
    app.set_is_btrfs(is_btrfs);
    app.set_fs_type_name(if is_btrfs { "BTRFS" } else { "EXT4/XFS" }.into());

    let snaps = SnapshotManager::list_snapshots();
    let slint_snaps: Vec<SnapshotItem> = snaps
        .into_iter()
        .map(|s| SnapshotItem {
            id: s.id.into(),
            name: s.name.into(),
            date: s.date.into(),
            description: s.description.into(),
            path: s.path.into(),
        })
        .collect();

    app.set_snapshot_list(ModelRc::new(VecModel::from(slint_snaps)));
}

fn refresh_store_page(app: &AppWindow, category: &str, query: &str) {
    let catalog = PackageManager::get_catalog();
    let filtered = PackageManager::filter_catalog(&catalog, category, query);

    let store_items: Vec<StoreItem> = filtered
        .into_iter()
        .map(|p| StoreItem {
            package_name: p.package_name.into(),
            display_name: p.display_name.into(),
            category: p.category.into(),
            description: p.description.into(),
            installed: p.installed,
        })
        .collect();

    app.set_store_items(ModelRc::new(VecModel::from(store_items)));
}
