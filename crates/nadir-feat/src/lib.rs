//! nadir-feat: wraps `SMILExtract` from openSMILE.
//!
//! We use openSMILE in two modes:
//! - **audit**: eGeMAPSv02 functionals across a full clip, returned as a map.
//! - **lld**: per-frame low-level descriptors (F0, loudness, MFCCs) for the pitch-audit loop.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SmileConfig {
    pub bin: PathBuf,
    pub config_dir: PathBuf,
}

impl Default for SmileConfig {
    fn default() -> Self {
        Self {
            bin: PathBuf::from("SMILExtract"),
            config_dir: PathBuf::from("tools/opensmile/config"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FeatureSet {
    EGeMAPSv02,
    ComParE2016,
    GeMAPSv01a,
    Emobase,
}

impl FeatureSet {
    pub fn config_path(self, root: &Path) -> PathBuf {
        let rel = match self {
            FeatureSet::EGeMAPSv02 => "egemaps/v02/eGeMAPSv02.conf",
            FeatureSet::ComParE2016 => "compare16/ComParE_2016.conf",
            FeatureSet::GeMAPSv01a => "gemaps/v01a/GeMAPSv01a.conf",
            FeatureSet::Emobase => "emobase/emobase.conf",
        };
        root.join(rel)
    }
}

/// Extract per-frame F0 LLD track using `prosody/smileF0.conf`. Emits CSV with
/// `name;frameTime;F0final_sma` columns — compatible with `parse_f0_track`.
pub fn extract_f0_lld(cfg: &SmileConfig, in_wav: &Path, out_csv: &Path) -> Result<()> {
    let conf = cfg.config_dir.join("prosody/smileF0.conf");
    let output = Command::new(&cfg.bin)
        .arg("-C")
        .arg(&conf)
        .arg("-I")
        .arg(in_wav)
        .arg("-csvoutput")
        .arg(out_csv)
        .arg("-loglevel")
        .arg("1")
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .with_context(|| format!("spawn SMILExtract at {}", cfg.bin.display()))?;
    let status = output.status;
    if !status.success() {
        anyhow::bail!(
            "SMILExtract smileF0 failed ({status}): {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub fn extract_csv(cfg: &SmileConfig, fs: FeatureSet, in_wav: &Path, out_csv: &Path) -> Result<()> {
    let conf = fs.config_path(&cfg.config_dir);
    let output = Command::new(&cfg.bin)
        .arg("-C")
        .arg(&conf)
        .arg("-I")
        .arg(in_wav)
        .arg("-csvoutput")
        .arg(out_csv)
        .arg("-loglevel")
        .arg("1")
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
        .with_context(|| format!("spawn SMILExtract at {}", cfg.bin.display()))?;
    let status = output.status;
    if !status.success() {
        anyhow::bail!(
            "SMILExtract failed ({status}): {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub fn tool_version(bin: &Path) -> Result<String> {
    let out = Command::new(bin).arg("-h").output().ok();
    Ok(match out {
        Some(o) => String::from_utf8_lossy(&o.stdout)
            .lines()
            .find(|l| l.contains("openSMILE"))
            .unwrap_or("openSMILE")
            .to_string(),
        None => "openSMILE (unknown)".into(),
    })
}

/// Extract per-frame F0 track from a CSV produced with an LLD-level config.
/// Expects a column named `F0final_sma` or `F0semitoneFrom27.5Hz_sma3nz`.
pub fn parse_f0_track(csv_text: &str) -> Vec<(f32, f32)> {
    let mut lines = csv_text.lines().filter(|l| !l.is_empty());
    let header = match lines.next() {
        Some(h) => h,
        None => return vec![],
    };
    let cols: Vec<&str> = header.split(';').collect();
    let Some(time_idx) = cols
        .iter()
        .position(|c| c.eq_ignore_ascii_case("frameTime"))
    else {
        return vec![];
    };
    let Some(f0_idx) = cols
        .iter()
        .position(|c| c.contains("F0final") || c.contains("F0semitone"))
    else {
        return vec![];
    };
    lines
        .filter_map(|l| {
            let parts: Vec<&str> = l.split(';').collect();
            let t: f32 = parts.get(time_idx)?.parse().ok()?;
            let f: f32 = parts.get(f0_idx)?.parse().ok()?;
            Some((t, f))
        })
        .collect()
}

/// RMS pitch error in cents between realized and target F0 tracks (matched by index).
pub fn rms_cents(realized: &[(f32, f32)], target: &[(f32, f32)]) -> f32 {
    let n = realized.len().min(target.len()).max(1) as f32;
    let sum: f32 = realized
        .iter()
        .zip(target)
        .map(|((_, r), (_, t))| {
            if *r <= 0.0 || *t <= 0.0 {
                0.0
            } else {
                let c = 1200.0 * (r / t).ln() / std::f32::consts::LN_2;
                c * c
            }
        })
        .sum();
    (sum / n).sqrt()
}

/// Fold `realized` frequency to the nearest octave of `target`. Absorbs F0
/// halving / doubling errors common in pitch trackers (ACF, SHS, etc.) so the
/// residual reflects actual tuning error rather than octave misassignment.
pub fn octave_fold(realized: f32, target: f32) -> f32 {
    if realized <= 0.0 || target <= 0.0 {
        return realized;
    }
    let mut r = realized;
    while r < target * 0.75 {
        r *= 2.0;
    }
    while r > target * 1.5 {
        r *= 0.5;
    }
    r
}

/// Trimmed RMS pitch error in cents, octave-folded and with the top
/// `trim_frac` of absolute errors excluded. Tracker glitches on transitions
/// produce a long tail of outlier frames that swamp the straight RMS; the
/// trimmed variant is closer to what a listener perceives (tiny transient
/// glitches vs. sustained detuning). `trim_frac = 0.05` drops the worst 5%.
pub fn rms_cents_trimmed(realized: &[(f32, f32)], target: &[(f32, f32)], trim_frac: f32) -> f32 {
    let n = realized.len().min(target.len());
    if n == 0 {
        return 0.0;
    }
    let mut errs: Vec<f32> = realized
        .iter()
        .zip(target)
        .filter_map(|((_, r), (_, t))| {
            if *r <= 0.0 || *t <= 0.0 {
                return None;
            }
            let folded = octave_fold(*r, *t);
            Some(1200.0 * (folded / t).ln() / std::f32::consts::LN_2)
        })
        .collect();
    if errs.is_empty() {
        return 0.0;
    }
    errs.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
    let keep = ((errs.len() as f32) * (1.0 - trim_frac.clamp(0.0, 0.5))).round() as usize;
    let keep = keep.max(1).min(errs.len());
    let sum: f32 = errs[..keep].iter().map(|e| e * e).sum();
    (sum / keep as f32).sqrt()
}

/// RMS pitch error in cents, but octave-folded so tracker halving/doubling
/// is not counted as tuning error. Use for audits against openSMILE when the
/// tracker is known to octave-slip.
pub fn rms_cents_octave_folded(realized: &[(f32, f32)], target: &[(f32, f32)]) -> f32 {
    let n = realized.len().min(target.len()).max(1) as f32;
    let sum: f32 = realized
        .iter()
        .zip(target)
        .map(|((_, r), (_, t))| {
            if *r <= 0.0 || *t <= 0.0 {
                0.0
            } else {
                let folded = octave_fold(*r, *t);
                let c = 1200.0 * (folded / t).ln() / std::f32::consts::LN_2;
                c * c
            }
        })
        .sum();
    (sum / n).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert!(parse_f0_track("").is_empty());
    }

    #[test]
    fn rms_cents_zero_when_match() {
        let a = vec![(0.0, 220.0), (0.01, 220.0)];
        assert!(rms_cents(&a, &a).abs() < 1e-3);
    }

    #[test]
    fn rms_cents_100() {
        let r = vec![(0.0, 220.0 * (2f32).powf(1.0 / 12.0))];
        let t = vec![(0.0, 220.0)];
        let c = rms_cents(&r, &t);
        assert!((c - 100.0).abs() < 1.0);
    }

    #[test]
    fn octave_fold_halves() {
        assert!((octave_fold(110.0, 220.0) - 220.0).abs() < 0.01);
        assert!((octave_fold(440.0, 220.0) - 220.0).abs() < 0.01);
        assert!((octave_fold(221.0, 220.0) - 221.0).abs() < 0.01);
    }

    #[test]
    fn folded_absorbs_halving() {
        let r = vec![(0.0, 220.0), (0.01, 110.0)];
        let t = vec![(0.0, 220.0), (0.01, 220.0)];
        assert!(rms_cents(&r, &t) > 800.0);
        assert!(rms_cents_octave_folded(&r, &t) < 1.0);
    }
}
