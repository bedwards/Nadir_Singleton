//! nadir CLI entrypoint.
//!
//! Surface: `nadir <subcmd> [args] -- [wrapped-tool args]`. `--help` at every level.

use anyhow::Result;
use clap::Parser;

mod cli;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    init_tracing(args.verbose);
    cli::dispatch(args)
}

fn init_tracing(verbose: u8) {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let level = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("nadir={level},nadir_core={level},nadir_vox={level},nadir_dsp={level},nadir_feat={level},nadir_praat={level},nadir_compose={level},nadir_render={level},nadir_vad={level}")));
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_timer(fmt::time::uptime()),
        )
        .with(filter)
        .init();
}
