use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
#[command(
    name = "hyperx-session",
    about = "Desktop Session Coordinator & Autostart Manager for HyperXOS",
    version = "1.0.0"
)]
struct Args {
    /// Launch session nested inside existing window manager (Winit backend)
    #[arg(short, long)]
    nested: bool,

    /// Custom command to run within session
    #[arg(short, long)]
    command: Option<String>,

    /// Print desktop session entry (.desktop) for display managers
    #[arg(long)]
    print_desktop_entry: bool,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    if args.print_desktop_entry {
        print_wayland_session_desktop_entry();
        return;
    }

    info!("============================================================");
    info!("  HyperXOS Pure Rust Desktop Environment Session (v1.0.0)");
    info!("============================================================");

    // 1. Export standard Wayland desktop environment variables
    setup_session_environment();

    // 2. Locate hyperx-comp and hyperx-shell binaries
    let comp_path = find_binary("hyperx-comp").unwrap_or_else(|| {
        PathBuf::from("/usr/bin/hyperx-comp")
    });
    let shell_path = find_binary("hyperx-shell").unwrap_or_else(|| {
        PathBuf::from("/usr/bin/hyperx-shell")
    });

    info!("Compositor binary: {:?}", comp_path);
    info!("Shell binary:      {:?}", shell_path);

    // 3. Launch Compositor (Dual-Tier: nested Winit vs bare-metal DRM)
    let has_display = env::var("WAYLAND_DISPLAY").is_ok() || env::var("DISPLAY").is_ok();
    let use_nested = args.nested || has_display;

    let mut comp_child = if use_nested {
        info!("Running in nested window mode (Winit backend)...");
        let mut comp_cmd = Command::new(&comp_path);
        comp_cmd.arg("--winit");
        match comp_cmd.spawn() {
            Ok(child) => {
                info!("Spawned hyperx-comp (PID: {})", child.id());
                child
            }
            Err(e) => {
                error!("Failed to launch compositor 'hyperx-comp': {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Bare-metal DRM session from SDDM / TTY
        info!("Running on bare-metal DRM display...");
        if let Some(kwin_bin) = find_binary("kwin_wayland") {
            info!("Launching hardware DRM Wayland backend via kwin_wayland...");
            match Command::new(kwin_bin)
                .args(["--drm", "--no-lockscreen", "--no-global-shortcuts", "--locale1"])
                .spawn()
            {
                Ok(child) => {
                    info!("Spawned DRM compositor (PID: {})", child.id());
                    child
                }
                Err(e) => {
                    warn!("Failed to spawn kwin_wayland, falling back to hyperx-comp: {}", e);
                    Command::new(&comp_path).spawn().expect("Failed to spawn hyperx-comp")
                }
            }
        } else {
            Command::new(&comp_path).spawn().expect("Failed to spawn hyperx-comp")
        }
    };

    // 4. Wait for Wayland display socket to become ready
    let runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let socket_name = wait_for_wayland_socket(&runtime_dir, Duration::from_secs(6))
        .unwrap_or_else(|| "wayland-0".to_string());

    info!("Wayland socket ready: {}", socket_name);
    env::set_var("WAYLAND_DISPLAY", &socket_name);

    // 5. Update DBus / systemd activation environment if available
    let _ = Command::new("dbus-update-activation-environment")
        .args(["--systemd", "WAYLAND_DISPLAY", "XDG_CURRENT_DESKTOP"])
        .spawn();

    // 6. Launch HyperX Desktop Shell (macOS Top Bar, Dynamic Island, Dock, Control Center)
    thread::sleep(Duration::from_millis(500));
    let mut shell_child = if shell_path.exists() {
        match Command::new(&shell_path).spawn() {
            Ok(child) => {
                info!("Spawned hyperx-shell (PID: {})", child.id());
                Some(child)
            }
            Err(e) => {
                warn!("Could not launch hyperx-shell: {}", e);
                None
            }
        }
    } else {
        warn!("hyperx-shell binary not found at {:?}", shell_path);
        None
    };

    // 7. Launch custom command or autostart if specified
    let mut custom_child = if let Some(ref cmd) = args.command {
        info!("Launching custom session command: {}", cmd);
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if let Some((bin, args)) = parts.split_first() {
            Command::new(bin).args(args).spawn().ok()
        } else {
            None
        }
    } else {
        None
    };

    // 8. Session Supervision Loop: monitor compositor lifecycle
    info!("Session initialized successfully. Monitoring session lifecycle...");
    loop {
        match comp_child.try_wait() {
            Ok(Some(status)) => {
                info!("Compositor exited with status: {}", status);
                break;
            }
            Ok(None) => {
                // Compositor is still running
                thread::sleep(Duration::from_millis(250));
            }
            Err(e) => {
                error!("Error monitoring compositor: {}", e);
                break;
            }
        }
    }

    // 9. Graceful Teardown: terminate shell and remaining child processes
    info!("Cleaning up HyperXOS desktop session processes...");
    if let Some(ref mut shell) = shell_child {
        terminate_child(shell);
    }
    if let Some(ref mut custom) = custom_child {
        terminate_child(custom);
    }

    info!("HyperXOS desktop session terminated cleanly.");
}

fn setup_session_environment() {
    env::set_var("XDG_CURRENT_DESKTOP", "HyperX:wlroots");
    env::set_var("XDG_SESSION_TYPE", "wayland");
    env::set_var("XDG_SESSION_DESKTOP", "HyperX");
    env::set_var("MOZ_ENABLE_WAYLAND", "1");
    env::set_var("QT_QPA_PLATFORM", "wayland;xcb");
    env::set_var("GDK_BACKEND", "wayland,x11");
    env::set_var("CLUTTER_BACKEND", "wayland");
    env::set_var("SDL_VIDEODRIVER", "wayland");

    info!("Session environment variables configured (HyperX:wlroots).");
}

fn find_binary(bin_name: &str) -> Option<PathBuf> {
    // 1. Current working directory or target/release build
    let target_release = Path::new("target/release").join(bin_name);
    if target_release.exists() {
        return Some(target_release);
    }

    let workspace_target = Path::new("rust/hyperx-de/target/release").join(bin_name);
    if workspace_target.exists() {
        return Some(workspace_target);
    }

    // 2. PATH resolution
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let candidate = dir.join(bin_name);
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    None
}

fn wait_for_wayland_socket(runtime_dir: &str, timeout: Duration) -> Option<String> {
    let start = Instant::now();
    let r_path = Path::new(runtime_dir);

    while start.elapsed() < timeout {
        for i in 0..10 {
            let sock = format!("wayland-{}", i);
            if r_path.join(&sock).exists() {
                return Some(sock);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    None
}

fn terminate_child(child: &mut Child) {
    let pid = Pid::from_raw(child.id() as i32);
    let _ = kill(pid, Signal::SIGTERM);

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(2) {
        if let Ok(Some(_)) = child.try_wait() {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }

    // Force kill if not exited
    let _ = kill(pid, Signal::SIGKILL);
    let _ = child.wait();
}

fn print_wayland_session_desktop_entry() {
    println!(
        r#"[Desktop Entry]
Name=HyperXOS
Comment=HyperXOS Pure Rust Desktop Environment (Smithay & Iced)
Exec=hyperx-session
Type=Application
DesktopNames=HyperX;wlroots
"#
    );
}
