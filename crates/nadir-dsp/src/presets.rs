//! Pre-baked csdr pipeline factories.
//!
//! Every function returns a pure `Graph` — no I/O, no env probing. The preset
//! module is consumed both by the `nadir dsp preset` CLI and by
//! `src/bin/emit-presets.rs`, which writes representative TOMLs under
//! `presets/` at the repo root.
//!
//! ## Frequency conventions
//!
//! csdr has two families of rate arguments and we stay strict about which is
//! which:
//!
//! * **Normalized frequency** (`rate = hz / fs`, range `-0.5..0.5`) — used by
//!   `shift_addition_cc`, `shift_addition_fc`, `bandpass_fir_fft_cc`,
//!   `peaks_fir_cc`. In this crate we pass these as `f32` already normalized.
//!   Factories that accept Hz convert internally against an explicit
//!   `sample_rate` and document the conversion in the doc comment.
//! * **Sample rate in Hz** — used by `deemphasis_wfm_ff`, `deemphasis_nfm_ff`
//!   (as "predefined rate"), and by any factory that emits the rate-dependent
//!   transition-bandwidth for `bandpass_fir_fft_cc`.
//!
//! Every factory documents its assumed sample rate. Where a factory accepts a
//! parameter in Hz, it performs the Hz→normalized conversion internally so
//! callers never have to think about it.
//!
//! ## Factory catalogue
//!
//! | factory | purpose |
//! |---|---|
//! | [`upsample_16_to_48`] | 16 kHz s16 → 48 kHz f32 |
//! | [`upsample_48_to_96`] | 48 kHz f32 → N× rate for speculative resampling |
//! | [`band_limit`] | bandpass FIR around a voice/instrument formant |
//! | [`ring_mod`] | single-carrier ring-mod at normalized `rate` |
//! | [`ring_mod_multi`] | cascaded ring-mods for inharmonic timbres |
//! | [`granular_texture`] | `dsb_fc` envelope-chopped grains at a fixed grain rate |
//! | [`shaped_noise_bed`] | uniform noise → FIR shape → bandlimit |
//! | [`dirac_impulse_bed`] | periodic click source for IR excitation |
//! | [`fir_cascade`] | chain of bandpass FIRs — the "FIR psalms" motif |
//! | [`deemphasis_chain`] | `deemphasis_nfm_ff` + gentle low-shelf |
//! | [`agc_limit_safe`] | `fastagc_ff | limit_ff` bus limiter |

use crate::{Graph, Stage};

/// Project master sample rate (Hz). All float-domain presets assume this
/// unless explicitly upsampling/downsampling.
pub const MASTER_SR: u32 = 48_000;

/// Upsample raw s16 16 kHz mono → f32 48 kHz mono.
///
/// Stages: `convert_s16_f | fir_interpolate_cc 3`.
pub fn upsample_16_to_48(bin: &str) -> Graph {
    Graph {
        in_sr: 16_000,
        out_sr: 48_000,
        bin: bin.into(),
        stages: vec![
            Stage::new("convert_s16_f"),
            Stage::new("fir_interpolate_cc").arg("3"),
        ],
    }
}

/// Upsample f32 48 kHz mono by integer `factor` (e.g. `2` → 96 kHz,
/// `4` → 192 kHz). Used for speculative high-rate side-chains before a later
/// downsample back to master rate.
///
/// Stages: `shift_addition_fc 0.0 | fir_interpolate_cc <factor> | realpart_cf`.
/// The `shift_addition_fc 0.0` is the idiomatic way to promote real-to-complex
/// in csdr without frequency translation; `realpart_cf` drops back to real.
pub fn upsample_48_to_96(factor: u32) -> Graph {
    Graph {
        in_sr: 48_000,
        out_sr: 48_000 * factor,
        bin: "csdr".into(),
        stages: vec![
            Stage::new("shift_addition_fc").arg("0.0"),
            Stage::new("fir_interpolate_cc").arg(format!("{factor}")),
            Stage::new("realpart_cf"),
        ],
    }
}

/// Band-limit a mono complex float stream using an overlap-add FFT FIR.
///
/// `low` and `high` are **normalized** frequencies (hz / fs, range `-0.5..0.5`).
/// Transition bandwidth is fixed at `0.005` (≈ 240 Hz at 48 kHz).
pub fn band_limit(low: f32, high: f32) -> Graph {
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![Stage::new("bandpass_fir_fft_cc")
            .arg(format!("{low}"))
            .arg(format!("{high}"))
            .arg("0.005")],
    }
}

/// Single-carrier ring-mod via `shift_addition_cc`.
///
/// `ratio` is the **normalized carrier rate** (`carrier_hz / fs`). Caller must
/// pre-divide; for example a 40 Hz carrier at 48 kHz master is
/// `40.0 / 48_000.0 ≈ 0.000833`.
pub fn ring_mod(ratio: f32) -> Graph {
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![Stage::new("shift_addition_cc").arg(format!("{ratio}"))],
    }
}

/// Cascaded ring-mods for inharmonic, bell-like timbres.
///
/// Each carrier in `carriers_hz` becomes one `shift_addition_cc` stage. The
/// caller passes carriers **in Hz** against the master sample rate; the
/// factory divides internally. Stage count equals `carriers_hz.len()`.
pub fn ring_mod_multi(carriers_hz: &[f32]) -> Graph {
    let fs = MASTER_SR as f32;
    let stages = carriers_hz
        .iter()
        .map(|hz| Stage::new("shift_addition_cc").arg(format!("{}", hz / fs)))
        .collect();
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages,
    }
}

/// Granular-texture pipeline: `dsb_fc` envelope carrier shaped by a per-grain
/// amplitude window, then bandlimited, then dropped back to real.
///
/// * `grain_ms` — per-grain window in milliseconds. We translate it to a
///   normalized gating rate of `1000.0 / grain_ms / fs`.
/// * `density` — crude "gain" on the envelope (1.0 ≈ unity, >1 compresses).
///
/// This is the csdr recipe described in `research/csdr.md` §"Granular
/// texture": `dsb_fc` synthesizes an envelope-modulated complex stream, a
/// narrow bandpass masks out a formant region, and `realpart_cf` drops back
/// to real. A `gain_ff` at the end is where a controller would FIFO-drive a
/// time-varying chop.
pub fn granular_texture(grain_ms: u32, density: f32) -> Graph {
    let fs = MASTER_SR as f32;
    let grain_hz = 1000.0 / grain_ms.max(1) as f32;
    let grain_norm = grain_hz / fs;
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![
            Stage::new("dsb_fc").arg("0"),
            Stage::new("shift_addition_cc").arg(format!("{grain_norm}")),
            Stage::new("bandpass_fir_fft_cc")
                .arg("-0.08")
                .arg("0.08")
                .arg("0.005"),
            Stage::new("realpart_cf"),
            Stage::new("gain_ff").arg(format!("{density}")),
        ],
    }
}

/// Shaped noise bed: uniform white noise → FIR shape → bandpass limit.
///
/// `low` / `high` are **normalized** band edges; `tilt` is the de-emphasis
/// `tau` in seconds passed to `deemphasis_wfm_ff` (we reuse it as a
/// treble-softener, per `research/csdr.md`). Larger `tilt` → darker.
pub fn shaped_noise_bed(low: f32, high: f32, tilt: f32) -> Graph {
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![
            Stage::new("uniform_noise_f"),
            Stage::new("gain_ff").arg("0.3"),
            Stage::new("dcblock_ff"),
            Stage::new("shift_addition_fc").arg("0.0"),
            Stage::new("bandpass_fir_fft_cc")
                .arg(format!("{low}"))
                .arg(format!("{high}"))
                .arg("0.02"),
            Stage::new("realpart_cf"),
            Stage::new("deemphasis_wfm_ff")
                .arg("48000")
                .arg(format!("{tilt}")),
            Stage::new("limit_ff").arg("0.9"),
        ],
    }
}

/// Dirac-train impulse bed: a periodic click generator useful for impulse
/// response excitation and as the "room impulse" motif across the corpus.
///
/// `rate_hz` is the click rate in Hz (e.g. `8.0` → 8 clicks/sec). We build it
/// as a DC source at unity multiplied by a narrow peak filter at the click
/// rate; in practice csdr has no clean impulse generator, so the compromise
/// is a `yes_f 1.0` DC feed into a `peaks_fir_cc` tuned to `rate_hz` —
/// the peak filter rings at that normalized rate on each restart of the DC
/// stream, giving a periodic excitation suitable for IR capture.
pub fn dirac_impulse_bed(rate_hz: f32) -> Graph {
    let fs = MASTER_SR as f32;
    let norm = rate_hz / fs;
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![
            Stage::new("yes_f").arg("1.0").arg("1"),
            Stage::new("shift_addition_fc").arg("0.0"),
            Stage::new("peaks_fir_cc")
                .arg("1024")
                .arg(format!("{norm}")),
            Stage::new("realpart_cf"),
            Stage::new("gain_ff").arg("0.5"),
        ],
    }
}

/// FIR cascade: chain one `bandpass_fir_fft_cc` per `(low, high)` band. The
/// "FIR psalms" motif is a deliberately stacked series of narrow bandpass
/// filters — each band inherits the previous stage's output, producing a
/// progressively narrower residue that sings.
///
/// Band edges are **normalized**. Stage count equals `bands.len()`.
pub fn fir_cascade(bands: &[(f32, f32)]) -> Graph {
    let stages = bands
        .iter()
        .map(|(low, high)| {
            Stage::new("bandpass_fir_fft_cc")
                .arg(format!("{low}"))
                .arg(format!("{high}"))
                .arg("0.005")
        })
        .collect();
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages,
    }
}

/// De-emphasis chain: `deemphasis_nfm_ff 48000` followed by a gentle
/// low-shelf expressed as a wide-band `bandpass_fir_fft_cc` with a soft
/// rolloff. The first stage is the standard NFM de-emphasis IIR; the second
/// is a complex bandpass acting as a low-shelf on the now-flatter spectrum.
///
/// Requires upstream real→complex promotion if fed from a raw vocal; this
/// factory assumes the caller already has a complex `_cc` stream at the
/// boundary of the second stage (typical in Nadir pipelines where earlier
/// stages produce `_c`).
pub fn deemphasis_chain() -> Graph {
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![
            Stage::new("deemphasis_nfm_ff").arg("48000"),
            Stage::new("shift_addition_fc").arg("0.0"),
            Stage::new("bandpass_fir_fft_cc")
                .arg("-0.45")
                .arg("0.2")
                .arg("0.05"),
            Stage::new("realpart_cf"),
        ],
    }
}

/// Final-bus safe limiter: `fastagc_ff` into `limit_ff`. The AGC rides the
/// envelope to a stable reference, the limiter clips at `±0.98`.
pub fn agc_limit_safe() -> Graph {
    Graph {
        in_sr: 48_000,
        out_sr: 48_000,
        bin: "csdr".into(),
        stages: vec![
            Stage::new("fastagc_ff").arg("1024").arg("0.5"),
            Stage::new("limit_ff").arg("0.98"),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build one graph per factory with representative arguments. This is the
    /// canonical list consumed by round-trip + non-empty tests and by the
    /// `emit_presets` binary.
    fn all_presets() -> Vec<(&'static str, Graph)> {
        vec![
            ("upsample_16_to_48", upsample_16_to_48("csdr")),
            ("upsample_48_to_96", upsample_48_to_96(2)),
            ("band_limit", band_limit(0.01, 0.4)),
            ("ring_mod", ring_mod(0.001)),
            (
                "ring_mod_multi",
                ring_mod_multi(&[0.001, 0.002, 0.003, 0.004]),
            ),
            ("granular_texture", granular_texture(40, 1.0)),
            ("shaped_noise_bed", shaped_noise_bed(-0.2, 0.2, 50e-6)),
            ("dirac_impulse_bed", dirac_impulse_bed(8.0)),
            (
                "fir_cascade",
                fir_cascade(&[(0.01, 0.1), (0.1, 0.3), (0.3, 0.4)]),
            ),
            ("deemphasis_chain", deemphasis_chain()),
            ("agc_limit_safe", agc_limit_safe()),
        ]
    }

    #[test]
    fn every_preset_roundtrips_through_toml() {
        for (name, g) in all_presets() {
            let s = g.to_toml().expect("to_toml");
            let g2 = Graph::parse_toml(&s).expect("parse_toml");
            assert_eq!(g.stages, g2.stages, "stages mismatch for {name}");
            assert_eq!(g.in_sr, g2.in_sr, "in_sr mismatch for {name}");
            assert_eq!(g.out_sr, g2.out_sr, "out_sr mismatch for {name}");
            assert_eq!(g.bin, g2.bin, "bin mismatch for {name}");
        }
    }

    #[test]
    fn every_preset_has_at_least_one_stage() {
        for (name, g) in all_presets() {
            assert!(!g.stages.is_empty(), "{name} has no stages");
        }
    }

    #[test]
    fn fir_cascade_stage_count_matches_bands() {
        let g = fir_cascade(&[(0.01, 0.1), (0.1, 0.3), (0.3, 0.4)]);
        assert_eq!(g.stages.len(), 3);
        for s in &g.stages {
            assert_eq!(s.cmd, "bandpass_fir_fft_cc");
        }
    }

    #[test]
    fn ring_mod_multi_stage_count_matches_carriers() {
        let g = ring_mod_multi(&[0.001, 0.002, 0.003, 0.004]);
        assert_eq!(g.stages.len(), 4);
        for s in &g.stages {
            assert_eq!(s.cmd, "shift_addition_cc");
        }
    }

    #[test]
    fn to_shell_starts_with_csdr_space() {
        for (name, g) in all_presets() {
            let sh = g.to_shell();
            assert!(sh.starts_with("csdr "), "{name} to_shell = {sh:?}");
        }
    }
}
