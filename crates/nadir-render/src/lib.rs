//! nadir-render: orchestrates the five tools into a track render.
//!
//! Concrete helpers here are shell-out plumbing (csdr pipelines + hound WAV I/O).
//! The audio math itself (noise generation, shaping, limiting) stays inside csdr.

use anyhow::{Context, Result};
use nadir_core::{RenderLock, Score};
use nadir_dsp::presets;
use std::io::Write;
use std::path::{Path, PathBuf};

pub const MASTER_SR: u32 = 48_000;

pub struct RenderPlan {
    pub out_dir: String,
    pub score: Score,
    pub seed: u64,
}

pub fn plan(out_dir: impl Into<String>, score: Score, seed: u64) -> RenderPlan {
    RenderPlan {
        out_dir: out_dir.into(),
        score,
        seed,
    }
}

pub fn render(_plan: &RenderPlan, _out_mix: &Path) -> Result<RenderLock> {
    let lock = RenderLock::default();
    Ok(lock)
}

/// Load a mono WAV (any common depth) into f32 samples in [-1,1] and return its sample rate.
pub fn wav_to_f32(path: &Path) -> Result<(Vec<f32>, u32)> {
    let reader = hound::WavReader::open(path).with_context(|| format!("open wav {path:?}"))?;
    let spec = reader.spec();
    let sr = spec.sample_rate;
    let ch = spec.channels as usize;
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            let bits = spec.bits_per_sample as i32;
            let max = (1i64 << (bits - 1)) as f32;
            hound::WavReader::open(path)?
                .into_samples::<i32>()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|v| v as f32 / max)
                .collect()
        }
        hound::SampleFormat::Float => hound::WavReader::open(path)?
            .into_samples::<f32>()
            .collect::<Result<Vec<_>, _>>()?,
    };
    let mono: Vec<f32> = if ch == 1 {
        samples
    } else {
        samples
            .chunks(ch)
            .map(|c| c.iter().copied().sum::<f32>() / ch as f32)
            .collect()
    };
    Ok((mono, sr))
}

/// Write f32 mono samples as 16-bit PCM WAV at `sr`.
pub fn f32_to_wav_s16(samples: &[f32], sr: u32, out: &Path) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sr,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(out, spec)?;
    for &s in samples {
        let v = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
        w.write_sample(v)?;
    }
    w.finalize()?;
    Ok(())
}

/// Write raw f32 little-endian bytes (for csdr input).
pub fn f32_to_raw(samples: &[f32], out: &Path) -> Result<()> {
    let mut f = fs_err::File::create(out)?;
    for &s in samples {
        f.write_all(&s.to_le_bytes())?;
    }
    Ok(())
}

/// Read raw f32 LE file.
pub fn raw_to_f32(path: &Path) -> Result<Vec<f32>> {
    let bytes = fs_err::read(path)?;
    let mut v = Vec::with_capacity(bytes.len() / 4);
    for ch in bytes.chunks_exact(4) {
        v.push(f32::from_le_bytes([ch[0], ch[1], ch[2], ch[3]]));
    }
    Ok(v)
}

/// Resample a 16 kHz s16 WAV to 48 kHz f32 raw using csdr's upsample_16_to_48 preset.
/// Returns the raw f32 samples at 48 kHz.
pub fn upsample_16_to_48_via_csdr(in_wav_16k: &Path) -> Result<Vec<f32>> {
    // Strip WAV header by reading with hound then writing raw s16.
    let (mono_f32, sr) = wav_to_f32(in_wav_16k)?;
    if sr != 16_000 {
        anyhow::bail!("expected 16 kHz input; got {sr}");
    }
    let raw_s16 = tempfile::NamedTempFile::with_suffix(".s16")?;
    {
        let mut f = fs_err::File::create(raw_s16.path())?;
        for s in &mono_f32 {
            let v = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
            f.write_all(&v.to_le_bytes())?;
        }
    }
    let g = presets::upsample_16_to_48("csdr");
    let out = tempfile::NamedTempFile::with_suffix(".f32")?;
    g.run_files(raw_s16.path(), out.path())?;
    raw_to_f32(out.path())
}

/// Generate a shaped noise bed of `duration_s` seconds at 48 kHz via csdr.
///
/// Pipeline (stdin = /dev/null; duration bounded by byte cap):
///   uniform_noise_f | gain_ff 0.3 | dcblock_ff | shift_addition_fc 0.0 |
///   bandpass_fir_fft_cc low high 0.02 | realpart_cf | deemphasis_wfm_ff 48000 tilt |
///   limit_ff 0.9
pub fn bed_shaped_noise(duration_s: f32, low: f32, high: f32, tilt: f32) -> Result<Vec<f32>> {
    let n_samples = (duration_s * MASTER_SR as f32).ceil() as usize;
    let max_bytes = n_samples * 4;
    let g = presets::shaped_noise_bed(low, high, tilt);
    let out = tempfile::NamedTempFile::with_suffix(".f32")?;
    g.run_generator(max_bytes, out.path())?;
    let mut samples = raw_to_f32(out.path())?;
    samples.truncate(n_samples);
    if samples.len() < n_samples {
        samples.resize(n_samples, 0.0);
    }
    Ok(samples)
}

/// Mix vocal (16 kHz WAV) + bed (48 kHz f32 samples) to a 48 kHz mono s16 WAV.
/// Vocal is first upsampled via csdr; then vocal_gain * vocal + bed_gain * bed.
pub fn mix_vocal_plus_bed_to_wav(
    vocal_wav_16k: &Path,
    bed_48k: &[f32],
    vocal_gain: f32,
    bed_gain: f32,
    out_wav: &Path,
) -> Result<PathBuf> {
    let vocal_48k = upsample_16_to_48_via_csdr(vocal_wav_16k)?;
    let n = vocal_48k.len().max(bed_48k.len());
    let mut mixed = Vec::with_capacity(n);
    for i in 0..n {
        let v = vocal_48k.get(i).copied().unwrap_or(0.0);
        let b = bed_48k.get(i).copied().unwrap_or(0.0);
        mixed.push(vocal_gain * v + bed_gain * b);
    }
    // Simple peak normalization if clipping
    let peak = mixed.iter().fold(0.0f32, |a, &x| a.max(x.abs()));
    if peak > 0.99 {
    let g = 0.99 / peak;
        for s in &mut mixed {
            *s *= g;
        }
    }
    f32_to_wav_s16(&mixed, MASTER_SR, out_wav)?;
    Ok(out_wav.to_path_buf())
}

/// Convenience: resolve a `bed_preset` string from a manifest to actual shaped-noise params.
/// Returns `None` for unrecognized presets (so caller can no-op or warn).
pub fn resolve_bed_preset(name: &str) -> Option<(f32, f32, f32)> {
    // (low_norm, high_norm, tilt_s)
    match name {
        "shaped_noise_dawn" => Some((-0.04, 0.04, 120e-6)),
        "shaped_noise_dusk" => Some((-0.08, 0.08, 200e-6)),
        "shaped_noise_air" => Some((-0.25, 0.25, 30e-6)),
        "band_limit_80_3200" => Some((80.0 / MASTER_SR as f32, 3200.0 / MASTER_SR as f32, 80e-6)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_raw_roundtrip() {
        let tmp = tempfile::NamedTempFile::with_suffix(".f32").unwrap();
        let src = vec![0.0f32, 0.5, -0.5, 1.0, -1.0];
        f32_to_raw(&src, tmp.path()).unwrap();
        let back = raw_to_f32(tmp.path()).unwrap();
        assert_eq!(src, back);
    }

    #[test]
    fn resolve_known_preset() {
        assert!(resolve_bed_preset("shaped_noise_dawn").is_some());
        assert!(resolve_bed_preset("unknown_xyz").is_none());
    }
}
