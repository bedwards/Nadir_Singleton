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
    let status = Command::new(&cfg.bin)
        .arg("-C")
        .arg(&conf)
        .arg("-I")
        .arg(in_wav)
        .arg("-csvoutput")
        .arg(out_csv)
        .status()
        .with_context(|| format!("spawn SMILExtract at {}", cfg.bin.display()))?;
    if !status.success() {
        anyhow::bail!("SMILExtract smileF0 failed ({status})");
    }
    Ok(())
}

pub fn extract_csv(cfg: &SmileConfig, fs: FeatureSet, in_wav: &Path, out_csv: &Path) -> Result<()> {
    let conf = fs.config_path(&cfg.config_dir);
    let status = Command::new(&cfg.bin)
        .arg("-C")
        .arg(&conf)
        .arg("-I")
        .arg(in_wav)
        .arg("-csvoutput")
        .arg(out_csv)
        .status()
        .with_context(|| format!("spawn SMILExtract at {}", cfg.bin.display()))?;
    if !status.success() {
        anyhow::bail!("SMILExtract failed ({status})");
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
}
