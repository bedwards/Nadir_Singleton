//! nadir-vad: subprocess bridge to `uv run python -m nadir_vad.cli ...`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct VadConfig {
    pub uv_bin: PathBuf,
    pub python_project_dir: PathBuf,
    pub threshold: f32,
    pub min_speech_ms: u32,
    pub min_silence_ms: u32,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            uv_bin: PathBuf::from("uv"),
            python_project_dir: PathBuf::from("python/nadir-vad"),
            threshold: 0.3,
            min_speech_ms: 60,
            min_silence_ms: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub start_s: f32,
    pub end_s: f32,
    pub prob: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Onset {
    pub time_s: f32,
    pub prob: f32,
    pub beat_index: Option<u32>,
}

pub fn detect_onsets(cfg: &VadConfig, in_wav: &Path, bpm: Option<f32>) -> Result<Vec<Onset>> {
    let mut cmd = Command::new(&cfg.uv_bin);
    cmd.arg("run")
        .arg("--project")
        .arg(&cfg.python_project_dir)
        .arg("python")
        .arg("-m")
        .arg("nadir_vad.cli")
        .arg("onsets")
        .arg("--input")
        .arg(in_wav)
        .arg("--threshold")
        .arg(format!("{}", cfg.threshold));
    if let Some(b) = bpm {
        cmd.arg("--bpm").arg(format!("{b}"));
    }
    let out = cmd
        .output()
        .with_context(|| format!("spawn uv at {}", cfg.uv_bin.display()))?;
    if !out.status.success() {
        anyhow::bail!(
            "nadir_vad onsets failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let onsets: Vec<Onset> =
        serde_json::from_str(&text).with_context(|| format!("parse onsets stdout: {text}"))?;
    Ok(onsets)
}

pub fn split_segments(cfg: &VadConfig, in_wav: &Path, out_dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let out = Command::new(&cfg.uv_bin)
        .arg("run")
        .arg("--project")
        .arg(&cfg.python_project_dir)
        .arg("python")
        .arg("-m")
        .arg("nadir_vad.cli")
        .arg("split")
        .arg("--input")
        .arg(in_wav)
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--threshold")
        .arg(format!("{}", cfg.threshold))
        .arg("--min-speech-ms")
        .arg(format!("{}", cfg.min_speech_ms))
        .arg("--min-silence-ms")
        .arg(format!("{}", cfg.min_silence_ms))
        .output()
        .with_context(|| format!("spawn uv at {}", cfg.uv_bin.display()))?;
    if !out.status.success() {
        anyhow::bail!(
            "nadir_vad split failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let paths: Vec<String> =
        serde_json::from_str(&text).with_context(|| format!("parse split stdout: {text}"))?;
    Ok(paths.into_iter().map(std::path::PathBuf::from).collect())
}

pub fn detect_segments(cfg: &VadConfig, in_wav: &Path) -> Result<Vec<Segment>> {
    let out = Command::new(&cfg.uv_bin)
        .arg("run")
        .arg("--project")
        .arg(&cfg.python_project_dir)
        .arg("python")
        .arg("-m")
        .arg("nadir_vad.cli")
        .arg("segments")
        .arg("--input")
        .arg(in_wav)
        .arg("--threshold")
        .arg(format!("{}", cfg.threshold))
        .arg("--min-speech-ms")
        .arg(format!("{}", cfg.min_speech_ms))
        .arg("--min-silence-ms")
        .arg(format!("{}", cfg.min_silence_ms))
        .output()
        .with_context(|| format!("spawn uv at {}", cfg.uv_bin.display()))?;
    if !out.status.success() {
        anyhow::bail!(
            "nadir_vad failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let segs: Vec<Segment> =
        serde_json::from_str(&text).with_context(|| format!("parse nadir_vad stdout: {text}"))?;
    Ok(segs)
}
