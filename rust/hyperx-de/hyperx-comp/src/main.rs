use clap::Parser;

mod handlers;
mod state;
mod winit;

#[derive(Parser, Debug)]
#[command(name = "hyperx-comp", about = "Pure Rust Wayland Compositor for HyperXOS (hyperx-de)", version = "0.1.0")]
struct Args {
    /// Run compositor in nested Winit window for development & testing
    #[arg(short, long, default_value_t = true)]
    winit: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hyperx_comp=info,smithay=info".into()),
        )
        .init();

    let args = Args::parse();

    if args.winit {
        winit::run_winit()?;
    } else {
        eprintln!("DRM/KMS bare-metal backend requested. Please run via hyperx-session or with root/seat access.");
    }

    Ok(())
}
