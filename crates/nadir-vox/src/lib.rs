//! nadir-vox: wraps MBROLA for singing synthesis.
//!
//! Pipeline: [`nadir_core::PhoStream`] → `mbrola voicedb - out.wav` → 16 kHz mono WAV.
//! Upsampling to 48 kHz and any further shaping happens downstream in nadir-dsp /
//! nadir-praat.

use anyhow::{Context, Result};
use nadir_core::PhoStream;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct MbrolaConfig {
    /// Path to `mbrola` binary.
    pub bin: PathBuf,
    /// Directory containing voice databases (e.g. `us1/us1`).
    pub voices_dir: PathBuf,
    /// Voice id (e.g. `us1`, `us3`, `en1`).
    pub voice: String,
    /// Frequency ratio (1.0 = unchanged).
    pub freq_ratio: f32,
    /// Time ratio (1.0 = unchanged).
    pub time_ratio: f32,
    /// Volume ratio.
    pub volume: f32,
}

impl Default for MbrolaConfig {
    fn default() -> Self {
        Self {
            bin: PathBuf::from("mbrola"),
            voices_dir: PathBuf::from("tools/mbrola-voices"),
            voice: "us1".into(),
            freq_ratio: 1.0,
            time_ratio: 1.0,
            volume: 1.0,
        }
    }
}

impl MbrolaConfig {
    pub fn voice_path(&self) -> PathBuf {
        self.voices_dir.join(&self.voice).join(&self.voice)
    }

    pub fn argv(&self, out_wav: &Path) -> Vec<String> {
        vec![
            "-f".into(),
            format!("{:.4}", self.freq_ratio),
            "-t".into(),
            format!("{:.4}", self.time_ratio),
            "-v".into(),
            format!("{:.4}", self.volume),
            self.voice_path().to_string_lossy().into_owned(),
            "-".into(),
            out_wav.to_string_lossy().into_owned(),
        ]
    }
}

/// Synthesize a `.pho` stream to a WAV file at the voice's native sample rate (16 kHz).
pub fn synth_to_wav(cfg: &MbrolaConfig, pho: &PhoStream, out_wav: &Path) -> Result<()> {
    let argv = cfg.argv(out_wav);
    tracing::info!(?cfg.bin, ?argv, "mbrola invoke");
    let mut child = Command::new(&cfg.bin)
        .args(&argv)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning mbrola at {}", cfg.bin.display()))?;

    let stdin = child.stdin.as_mut().context("mbrola stdin")?;
    stdin.write_all(pho.to_string().as_bytes())?;
    drop(child.stdin.take());

    let out = child.wait_with_output()?;
    if !out.status.success() {
        anyhow::bail!(
            "mbrola failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(())
}

pub fn tool_version(bin: &Path) -> Result<String> {
    let out = Command::new(bin).arg("-h").output();
    match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            Ok(s.lines().next().unwrap_or("mbrola").to_string())
        }
        Err(_) => Ok("mbrola (unknown)".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nadir_core::Pho;

    #[test]
    fn argv_contains_voice_and_output() {
        let cfg = MbrolaConfig::default();
        let out = Path::new("/tmp/x.wav");
        let argv = cfg.argv(out);
        assert!(argv.iter().any(|a| a.contains("us1")));
        assert!(argv.iter().any(|a| a.ends_with("x.wav")));
    }

    #[test]
    fn pho_stream_renders() {
        let mut s = PhoStream::new();
        s.push(Pho::silence(50));
        s.push(Pho::voiced("a", 200, 220.0));
        assert!(s.to_string().contains("a 200"));
    }
}
