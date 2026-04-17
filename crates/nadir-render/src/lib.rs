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

/// Write stereo (L, R) f32 samples as 16-bit PCM WAV at `sr`.
/// Both slices must have the same length.
pub fn stereo_to_wav_s16(left: &[f32], right: &[f32], sr: u32, out: &Path) -> Result<()> {
    anyhow::ensure!(
        left.len() == right.len(),
        "stereo channels differ in length"
    );
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: sr,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(out, spec)?;
    for (l, r) in left.iter().zip(right.iter()) {
        let lv = (l.clamp(-1.0, 1.0) * 32767.0) as i16;
        let rv = (r.clamp(-1.0, 1.0) * 32767.0) as i16;
        w.write_sample(lv)?;
        w.write_sample(rv)?;
    }
    w.finalize()?;
    Ok(())
}

/// Apply a per-syllable amplitude envelope in-place. The syllable timing is
/// reconstructed from `note_dur_ms` (one entry per syllable) plus the breath
/// and intra-syllable silences that `render_vox_pho_phrased` inserts around
/// them, so the envelope lines up with the MBROLA-synthesized audio.
///
/// `stress_to_gain` is called per syllable to produce the target gain; the
/// envelope ramps between syllables over `xfade_ms` so there are no clicks.
#[allow(clippy::too_many_arguments)]
pub fn apply_syllable_dynamics(
    samples: &mut [f32],
    sr: u32,
    note_dur_ms: &[u32],
    phrase_lens: &[usize],
    stresses: &[f32],
    breath_ms: u32,
    intra_ms: u32,
    xfade_ms: u32,
    stress_to_gain: impl Fn(f32) -> f32,
) {
    let ms_to_samples = |ms: u32| -> usize { (ms as f32 * sr as f32 / 1000.0) as usize };
    let xfade = ms_to_samples(xfade_ms).max(1);
    // Build a piecewise-constant target gain per sample, then smooth on the fly.
    let mut target = vec![1.0f32; samples.len()];
    let mut cursor = ms_to_samples(breath_ms); // opening silence
    let mut syl_i = 0usize;
    for (p_idx, &plen) in phrase_lens.iter().enumerate() {
        for k in 0..plen {
            if syl_i >= note_dur_ms.len() {
                break;
            }
            let dur_samples = ms_to_samples(note_dur_ms[syl_i]);
            let stress = stresses.get(syl_i).copied().unwrap_or(1.0);
            let gain = stress_to_gain(stress);
            let end = (cursor + dur_samples).min(target.len());
            for v in target[cursor..end].iter_mut() {
                *v = gain;
            }
            cursor = end;
            if k + 1 < plen {
                cursor = (cursor + ms_to_samples(intra_ms)).min(target.len());
            }
            syl_i += 1;
        }
        // phrase-end breath
        let _ = p_idx;
        cursor = (cursor + ms_to_samples(breath_ms)).min(target.len());
    }
    // Smooth transitions with a simple one-pole filter
    let alpha = 1.0 / xfade as f32;
    let mut env = target[0];
    for (i, s) in samples.iter_mut().enumerate() {
        let t = target[i];
        env += alpha * (t - env);
        *s *= env;
    }
}

/// RMS level in dBFS of the non-silent portion of `samples`.
/// Samples below `silence_threshold` (default 0.001) are excluded so leading
/// /trailing quiet doesn't drag the measurement down. This is not true ITU BS.1770
/// LUFS (no K-weighting, no gating) — it's a first-order loudness proxy that
/// tracks well enough for stem balance within a single render.
pub fn rms_dbfs(samples: &[f32]) -> f32 {
    let thr = 0.001f32;
    let mut sum_sq = 0.0f64;
    let mut n = 0u64;
    for &s in samples {
        if s.abs() > thr {
            sum_sq += (s as f64) * (s as f64);
            n += 1;
        }
    }
    if n == 0 {
        return f32::NEG_INFINITY;
    }
    let rms = (sum_sq / n as f64).sqrt();
    20.0 * (rms as f32).log10()
}

/// Scale `samples` so measured RMS matches `target_dbfs`. If the input is
/// silent (or the requested gain would exceed `max_gain_db`), returns the
/// samples unchanged. Useful to normalize stems toward a target loudness
/// before mixing.
pub fn normalize_to_dbfs(samples: &mut [f32], target_dbfs: f32, max_gain_db: f32) {
    let measured = rms_dbfs(samples);
    if !measured.is_finite() {
        return;
    }
    let delta = target_dbfs - measured;
    let clamped = delta.clamp(-max_gain_db, max_gain_db);
    let gain = 10f32.powf(clamped / 20.0);
    for s in samples.iter_mut() {
        *s *= gain;
    }
}

/// Apply a sinusoidal amplitude envelope (tremolo) at `rate_hz` with `depth`
/// in [0, 1]: depth=0 → no modulation, depth=0.3 → swings 70%..100% of input.
/// Starts at phase 0 so the envelope begins near max (no attack click).
pub fn amp_tremolo(samples: &mut [f32], rate_hz: f32, depth: f32) {
    let sr = MASTER_SR as f32;
    let dphi = 2.0 * std::f32::consts::PI * rate_hz / sr;
    let mut phi = 0.0f32;
    let d = depth.clamp(0.0, 1.0);
    for s in samples.iter_mut() {
        let env = 1.0 - d * 0.5 * (1.0 - phi.cos());
        *s *= env;
        phi += dphi;
    }
}

/// Multi-tap delay: for each (delay_ms, gain) tap, add a shifted copy of
/// `samples` to the output. In-place style: returns a new Vec of length
/// `samples.len() + max_delay_samples` so the trailing echoes aren't truncated.
/// This is simple array arithmetic (shift + scale + sum) — the same category
/// of glue math as stem mixing, not a new DSP tool.
pub fn multi_tap_delay(samples: &[f32], sr: u32, taps: &[(u32, f32)]) -> Vec<f32> {
    let max_delay = taps.iter().map(|(ms, _)| *ms).max().unwrap_or(0);
    let max_delay_samples = (max_delay as f32 * sr as f32 / 1000.0) as usize;
    let n = samples.len() + max_delay_samples;
    let mut out = vec![0.0f32; n];
    for (i, &v) in samples.iter().enumerate() {
        out[i] += v;
    }
    for (ms, g) in taps {
        let off = (*ms as f32 * sr as f32 / 1000.0) as usize;
        for (i, &v) in samples.iter().enumerate() {
            let idx = i + off;
            if idx < n {
                out[idx] += g * v;
            }
        }
    }
    out
}

/// Equal-power pan gains. `pan` in [-1.0, 1.0]: -1 = full L, 0 = center, 1 = full R.
pub fn pan_gains(pan: f32) -> (f32, f32) {
    let p = pan.clamp(-1.0, 1.0);
    let theta = (p + 1.0) * std::f32::consts::FRAC_PI_4;
    (theta.cos(), theta.sin())
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

/// Synthesize a tonic-triad drone bed: sum of sinusoids at the key's
/// root / 3rd / 5th at the given octave. Duration in seconds; 48 kHz mono f32.
/// Applies a linear fade-in / fade-out of `fade_s` seconds. Detune the 3rd and
/// 5th by ±2 cents each for movement.
pub fn bed_tonal_triad(
    scale: &nadir_core::Scale,
    duration_s: f32,
    octave: i32,
    fade_s: f32,
) -> Vec<f32> {
    let degrees = scale.degrees_hz(octave);
    let root = degrees.first().copied().unwrap_or(220.0);
    let third = degrees
        .get(2)
        .copied()
        .unwrap_or(root * 2f32.powf(3.0 / 12.0));
    let fifth = degrees
        .get(4)
        .copied()
        .unwrap_or(root * 2f32.powf(7.0 / 12.0));
    let sr = MASTER_SR as f32;
    let n = (duration_s * sr).ceil() as usize;
    let mut out = vec![0.0f32; n];
    // Small cent detune to avoid a sterile sine stack
    let cents = |hz: f32, c: f32| hz * 2f32.powf(c / 1200.0);
    let partials: [(f32, f32); 3] = [
        (root, 0.45),
        (cents(third, -2.0), 0.30),
        (cents(fifth, 2.0), 0.30),
    ];
    for (f, amp) in partials {
        let dphi = 2.0 * std::f32::consts::PI * f / sr;
        let mut phi: f32 = 0.0;
        for s in out.iter_mut().take(n) {
            *s += amp * phi.sin();
            phi += dphi;
        }
    }
    // Fades
    let fn_samples = (fade_s * sr) as usize;
    for i in 0..fn_samples.min(n) {
        let g = i as f32 / fn_samples as f32;
        out[i] *= g;
        out[n - 1 - i] *= g;
    }
    out
}

/// Synthesize a noise-burst pulse track at `onsets_s` (seconds).
pub fn pulse_track(onsets_s: &[f32], duration_s: f32, pulse_ms: u32, seed: u64) -> Vec<f32> {
    let sr = MASTER_SR as f32;
    let n = (duration_s * sr).ceil() as usize;
    let mut out = vec![0.0f32; n];
    let plen = ((pulse_ms as f32 / 1000.0) * sr) as usize;
    let mut rng = seed;
    let mut rand01 = || {
        rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((rng >> 33) as f32) / ((1u32 << 31) as f32) * 2.0 - 1.0
    };
    for &t in onsets_s {
        let start = (t * sr) as usize;
        for i in 0..plen {
            let idx = start + i;
            if idx >= n {
                break;
            }
            let u = i as f32 / plen as f32;
            let env = (std::f32::consts::PI * u).sin().powi(2);
            out[idx] += 0.6 * env * rand01();
        }
    }
    out
}

/// Split `onsets_s` into even-indexed / odd-indexed subsequences.
/// Used to ping-pong a pulse track between L and R.
pub fn split_onsets_even_odd(onsets_s: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for (i, &t) in onsets_s.iter().enumerate() {
        if i % 2 == 0 {
            even.push(t);
        } else {
            odd.push(t);
        }
    }
    (even, odd)
}

/// Beat-grid hit times. `subdivision` = 1 → quarter notes, 2 → 8th, 4 → 16th.
/// Times start at 0 and continue every `60 / (bpm * subdivision)` s up to
/// `duration_s`.
pub fn beat_grid_times(bpm: f32, duration_s: f32, subdivision: u32) -> Vec<f32> {
    let step = 60.0 / (bpm.max(1.0) * subdivision.max(1) as f32);
    let mut out = Vec::new();
    let mut t = 0.0f32;
    while t < duration_s {
        out.push(t);
        t += step;
    }
    out
}

/// Cycle sine bursts through `hz_cycle` on a beat-grid subdivision. Each burst
/// has a fast-decay envelope and `note_ms` length. Used for bass arpeggios,
/// chord stabs, etc.
pub fn arp_track(
    hz_cycle: &[f32],
    duration_s: f32,
    bpm: f32,
    subdivision: u32,
    note_ms: u32,
) -> Vec<f32> {
    if hz_cycle.is_empty() {
        return Vec::new();
    }
    let sr = MASTER_SR as f32;
    let n = (duration_s * sr).ceil() as usize;
    let mut out = vec![0.0f32; n];
    let plen = ((note_ms as f32 / 1000.0) * sr) as usize;
    let times = beat_grid_times(bpm, duration_s, subdivision);
    for (i, &t) in times.iter().enumerate() {
        let hz = hz_cycle[i % hz_cycle.len()];
        let dphi = 2.0 * std::f32::consts::PI * hz / sr;
        let start = (t * sr) as usize;
        let mut phi = 0.0f32;
        for j in 0..plen {
            let idx = start + j;
            if idx >= n {
                break;
            }
            let u = j as f32 / plen as f32;
            let env = (1.0 - u).powi(2) * (1.0 - (-10.0 * u).exp());
            out[idx] += 0.6 * env * phi.sin();
            phi += dphi;
        }
    }
    out
}

/// Synthesize a pitched pulse track (sinusoid bursts) at `onsets_s`.
/// Each pulse is a fast-decaying sine at `hz` with pulse_ms envelope —
/// a kick-like tonal percussion. Used when `pulse_kind = "tonic"`.
pub fn pulse_track_pitched(onsets_s: &[f32], duration_s: f32, pulse_ms: u32, hz: f32) -> Vec<f32> {
    let sr = MASTER_SR as f32;
    let n = (duration_s * sr).ceil() as usize;
    let mut out = vec![0.0f32; n];
    let plen = ((pulse_ms as f32 / 1000.0) * sr) as usize;
    let dphi = 2.0 * std::f32::consts::PI * hz / sr;
    for &t in onsets_s {
        let start = (t * sr) as usize;
        let mut phi = 0.0f32;
        for i in 0..plen {
            let idx = start + i;
            if idx >= n {
                break;
            }
            let u = i as f32 / plen as f32;
            // exp decay envelope — fast attack, faster decay
            let env = (1.0 - u).powi(3) * (1.0 - (-8.0 * u).exp());
            out[idx] += 0.7 * env * phi.sin();
            phi += dphi;
        }
    }
    out
}

/// Run a raw f32 48 kHz mono stream through the csdr `agc_limit_safe` preset
/// (fastagc_ff 1024 0.5 | limit_ff 0.98). Returns the processed samples.
/// Use as a master-bus safety net before writing the final WAV.
pub fn master_agc_limit(samples: &[f32]) -> Result<Vec<f32>> {
    let tmp_in = tempfile::NamedTempFile::with_suffix(".f32")?;
    f32_to_raw(samples, tmp_in.path())?;
    let g = presets::agc_limit_safe();
    let tmp_out = tempfile::NamedTempFile::with_suffix(".f32")?;
    g.run_files(tmp_in.path(), tmp_out.path())?;
    raw_to_f32(tmp_out.path())
}

/// Band-limit a raw f32 48 kHz mono stream through csdr
/// (shift_addition_fc 0.0 | bandpass_fir_fft_cc low high tbw | realpart_cf).
pub fn band_limit_via_csdr(samples: &[f32], low: f32, high: f32, tbw: f32) -> Result<Vec<f32>> {
    let tmp_in = tempfile::NamedTempFile::with_suffix(".f32")?;
    f32_to_raw(samples, tmp_in.path())?;
    let g = nadir_dsp::Graph {
        in_sr: MASTER_SR,
        out_sr: MASTER_SR,
        bin: "csdr".into(),
        stages: vec![
            nadir_dsp::Stage::new("shift_addition_fc").arg("0.0"),
            nadir_dsp::Stage::new("bandpass_fir_fft_cc")
                .arg(format!("{low}"))
                .arg(format!("{high}"))
                .arg(format!("{tbw}")),
            nadir_dsp::Stage::new("realpart_cf"),
        ],
    };
    let tmp_out = tempfile::NamedTempFile::with_suffix(".f32")?;
    g.run_files(tmp_in.path(), tmp_out.path())?;
    raw_to_f32(tmp_out.path())
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
    mix_stems_to_wav(vocal_wav_16k, &[(bed_48k, bed_gain)], vocal_gain, out_wav)
}

/// General stem mixer. Vocal is upsampled via csdr. Each `(stem, gain)` is mixed in at 48 kHz.
/// The final sum runs through csdr agc_limit_safe (master bus) before WAV write.
pub fn mix_stems_to_wav(
    vocal_wav_16k: &Path,
    stems: &[(&[f32], f32)],
    vocal_gain: f32,
    out_wav: &Path,
) -> Result<PathBuf> {
    // Mono implementation preserved as shim: promotes every stem to center pan.
    let stems_panned: Vec<(&[f32], f32, f32)> = stems.iter().map(|(s, g)| (*s, *g, 0.0)).collect();
    mix_stems_stereo(vocal_wav_16k, &stems_panned, vocal_gain, 0.0, out_wav)
}

/// Stereo stem mixer without echo. See `mix_stems_stereo_with_echo`.
pub fn mix_stems_stereo(
    vocal_wav_16k: &Path,
    stems: &[(&[f32], f32, f32)],
    vocal_gain: f32,
    vocal_pan: f32,
    out_wav: &Path,
) -> Result<PathBuf> {
    mix_stems_stereo_with_echo(vocal_wav_16k, stems, vocal_gain, vocal_pan, &[], out_wav)
}

/// Stereo stem mixer. Vocal is upsampled via csdr and panned at `vocal_pan`.
/// Each `(stem, gain, pan)` is panned with equal-power law. Master bus AGC+limit
/// is applied independently per channel before WAV write. If `echo_taps` is
/// non-empty, the vocal (only) gets a multi-tap echo applied before panning,
/// widening the sense of space without affecting the pulse hit transients.
pub fn mix_stems_stereo_with_echo(
    vocal_wav_16k: &Path,
    stems: &[(&[f32], f32, f32)],
    vocal_gain: f32,
    vocal_pan: f32,
    echo_taps: &[(u32, f32)],
    out_wav: &Path,
) -> Result<PathBuf> {
    let vocal_48k = upsample_16_to_48_via_csdr(vocal_wav_16k)?;
    // Stereo echo: alternate taps L/R so reflections ping-pong. Index 0 → L,
    // 1 → R, 2 → L, etc. Direct signal (tap 0 position is always present in
    // both via multi_tap_delay's initial copy) stays centered.
    let (vocal_l, vocal_r): (Vec<f32>, Vec<f32>) = if echo_taps.is_empty() {
        (vocal_48k.clone(), vocal_48k.clone())
    } else {
        let taps_l: Vec<(u32, f32)> = echo_taps
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if i % 2 == 0 { Some(*t) } else { None })
            .collect();
        let taps_r: Vec<(u32, f32)> = echo_taps
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if i % 2 == 1 { Some(*t) } else { None })
            .collect();
        (
            multi_tap_delay(&vocal_48k, MASTER_SR, &taps_l),
            multi_tap_delay(&vocal_48k, MASTER_SR, &taps_r),
        )
    };
    let mut n = vocal_l.len().max(vocal_r.len());
    for (s, _, _) in stems {
        n = n.max(s.len());
    }
    let mut left = vec![0.0f32; n];
    let mut right = vec![0.0f32; n];
    let (vl, vr) = pan_gains(vocal_pan);
    for (i, v) in vocal_l.iter().enumerate() {
        left[i] += vocal_gain * vl * v;
    }
    for (i, v) in vocal_r.iter().enumerate() {
        right[i] += vocal_gain * vr * v;
    }
    for (stem, g, pan) in stems {
        let (pl, pr) = pan_gains(*pan);
        for (i, v) in stem.iter().enumerate() {
            if i >= n {
                break;
            }
            left[i] += g * pl * v;
            right[i] += g * pr * v;
        }
    }
    let l_final = master_agc_limit(&left).unwrap_or(left);
    let r_final = master_agc_limit(&right).unwrap_or(right);
    stereo_to_wav_s16(&l_final, &r_final, MASTER_SR, out_wav)?;
    Ok(out_wav.to_path_buf())
}

/// Bed shape: noise-based presets produce shaped noise; tonal presets draw
/// from key/scale. A manifest may name either — this enum lets callers decide.
pub enum BedKind {
    ShapedNoise { low: f32, high: f32, tilt: f32 },
    TonalTriad { octave: i32, fade_s: f32 },
}

/// Convenience: resolve a `bed_preset` string from a manifest to a BedKind.
/// Returns `None` for unrecognized presets.
pub fn resolve_bed(name: &str) -> Option<BedKind> {
    match name {
        "shaped_noise_dawn" => Some(BedKind::ShapedNoise {
            low: -0.04,
            high: 0.04,
            tilt: 120e-6,
        }),
        "shaped_noise_dusk" => Some(BedKind::ShapedNoise {
            low: -0.08,
            high: 0.08,
            tilt: 200e-6,
        }),
        "shaped_noise_air" => Some(BedKind::ShapedNoise {
            low: -0.25,
            high: 0.25,
            tilt: 30e-6,
        }),
        "band_limit_80_3200" => Some(BedKind::ShapedNoise {
            low: 80.0 / MASTER_SR as f32,
            high: 3200.0 / MASTER_SR as f32,
            tilt: 80e-6,
        }),
        "tonal_drone_triad" => Some(BedKind::TonalTriad {
            octave: -1,
            fade_s: 0.8,
        }),
        "tonal_drone_low" => Some(BedKind::TonalTriad {
            octave: -2,
            fade_s: 1.2,
        }),
        "tonal_drone_high" => Some(BedKind::TonalTriad {
            octave: 0,
            fade_s: 0.5,
        }),
        _ => None,
    }
}

/// Back-compat: noise-only shim for call sites that only support shaped noise.
pub fn resolve_bed_preset(name: &str) -> Option<(f32, f32, f32)> {
    match resolve_bed(name)? {
        BedKind::ShapedNoise { low, high, tilt } => Some((low, high, tilt)),
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
