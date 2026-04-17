//! Regenerate `presets/*.toml` at the workspace root.
//!
//! Calls every factory in [`nadir_dsp::presets`] with representative arguments
//! and writes each graph's `to_toml` output to `presets/<name>.toml`. Run from
//! any subdirectory of the workspace — we walk up from this crate's manifest
//! to find the workspace root.
//!
//! Invocation:
//!
//! ```sh
//! cargo run -p nadir-dsp --bin emit_presets
//! ```

use anyhow::{Context, Result};
use nadir_dsp::{presets, Graph};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let out_dir = presets_dir()?;
    fs_err::create_dir_all(&out_dir)
        .with_context(|| format!("create_dir_all {}", out_dir.display()))?;

    for (name, g) in catalogue() {
        write_preset(&out_dir, name, &g)?;
    }
    Ok(())
}

/// Resolve `<workspace_root>/presets/`. The workspace root is two levels up
/// from this crate's manifest (`crates/nadir-dsp` → `../..`).
fn presets_dir() -> Result<PathBuf> {
    let crate_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    let workspace_root = crate_manifest_dir
        .parent()
        .and_then(Path::parent)
        .with_context(|| format!("walk two up from {}", crate_manifest_dir.display()))?
        .to_path_buf();
    Ok(workspace_root.join("presets"))
}

fn write_preset(dir: &Path, name: &str, g: &Graph) -> Result<()> {
    let path = dir.join(format!("{name}.toml"));
    let body = g.to_toml().with_context(|| format!("to_toml for {name}"))?;
    fs_err::write(&path, body).with_context(|| format!("write {}", path.display()))?;
    println!("{}", path.display());
    Ok(())
}

/// Canonical `(name, graph)` list. Names become filenames under `presets/`.
fn catalogue() -> Vec<(&'static str, Graph)> {
    vec![
        ("upsample_16_to_48", presets::upsample_16_to_48("csdr")),
        ("upsample_48_to_96", presets::upsample_48_to_96(2)),
        ("band_limit", presets::band_limit(0.01, 0.4)),
        ("ring_mod", presets::ring_mod(0.001)),
        (
            "ring_mod_multi",
            presets::ring_mod_multi(&[0.001, 0.002, 0.003, 0.004]),
        ),
        ("granular_texture", presets::granular_texture(40, 1.0)),
        (
            "shaped_noise_bed",
            presets::shaped_noise_bed(-0.2, 0.2, 50e-6),
        ),
        ("dirac_impulse_bed", presets::dirac_impulse_bed(8.0)),
        (
            "fir_cascade",
            presets::fir_cascade(&[(0.01, 0.1), (0.1, 0.3), (0.3, 0.4)]),
        ),
        ("deemphasis_chain", presets::deemphasis_chain()),
        ("agc_limit_safe", presets::agc_limit_safe()),
    ]
}
