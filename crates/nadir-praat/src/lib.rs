//! nadir-praat: generate Praat scripts from Rust, run them headlessly.

use anyhow::{Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct PraatConfig {
    pub bin: PathBuf,
}

impl Default for PraatConfig {
    fn default() -> Self {
        Self {
            bin: PathBuf::from("praat"),
        }
    }
}

pub fn run_script(cfg: &PraatConfig, script: &Path, args: &[String]) -> Result<String> {
    let out = Command::new(&cfg.bin)
        .arg("--no-pref-files")
        .arg("--run")
        .arg(script)
        .args(args)
        .output()
        .with_context(|| format!("spawn praat at {}", cfg.bin.display()))?;
    if !out.status.success() {
        anyhow::bail!(
            "praat failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn tool_version(bin: &Path) -> Result<String> {
    let out = Command::new(bin).arg("--version").output().ok();
    Ok(match out {
        Some(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        None => "praat (unknown)".into(),
    })
}

/// Render a PSOLA-retarget script that resynthesises `in_wav` with F0 taken from
/// a pitch-tier pairs file `pitch_csv` (columns: `time_s,hz`) and writes to `out_wav`.
pub fn psola_retarget_script(in_wav: &Path, pitch_csv: &Path, out_wav: &Path) -> String {
    format!(
        r#"# nadir-praat: PSOLA retarget
Read from file: "{in_wav}"
name$ = selected$("Sound")
To Manipulation: 0.01, 75, 600
Create PitchTier: "target", 0, 1
table = Read Table from comma-separated file: "{pitch_csv}"
nrow = Get number of rows
for i from 1 to nrow
    selectObject: table
    t = Get value: i, "time_s"
    h = Get value: i, "hz"
    selectObject: "PitchTier target"
    Add point: t, h
endfor
selectObject: "Manipulation " + name$
plusObject: "PitchTier target"
Replace pitch tier
selectObject: "Manipulation " + name$
resyn = Get resynthesis (overlap-add)
selectObject: resyn
Save as WAV file: "{out_wav}"
"#,
        in_wav = in_wav.display(),
        pitch_csv = pitch_csv.display(),
        out_wav = out_wav.display(),
    )
}

/// Extract per-frame F0 track from a WAV via Praat SHS pitch analysis.
/// Returns a CSV with `time_s,hz` — unvoiced frames are omitted (hz==0 skipped).
pub fn extract_f0_script(in_wav: &Path, out_csv: &Path) -> String {
    format!(
        r#"# nadir-praat: F0 extraction
Read from file: "{in_wav}"
To Pitch (ac): 0.01, 75, 15, 0, 0.03, 0.45, 0.01, 0.35, 0.14, 600
writeFileLine: "{out_csv}", "time_s,hz"
frames = Get number of frames
for i from 1 to frames
    t = Get time from frame number: i
    h = Get value in frame: i, "Hertz"
    if h <> undefined
        appendFileLine: "{out_csv}", fixed$(t, 4) + "," + fixed$(h, 3)
    endif
endfor
"#,
        in_wav = in_wav.display(),
        out_csv = out_csv.display(),
    )
}

/// Write a script to a tempfile and run it.
pub fn run_inline(cfg: &PraatConfig, script_body: &str, args: &[String]) -> Result<String> {
    let mut tf = tempfile::NamedTempFile::new()?;
    tf.write_all(script_body.as_bytes())?;
    tf.flush()?;
    run_script(cfg, tf.path(), args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn script_contains_key_primitives() {
        let s = psola_retarget_script(
            Path::new("/tmp/in.wav"),
            Path::new("/tmp/p.csv"),
            Path::new("/tmp/out.wav"),
        );
        assert!(s.contains("To Manipulation"));
        assert!(s.contains("Replace pitch tier"));
        assert!(s.contains("overlap-add"));
        // No form block — paths baked via Rust string interpolation
        assert!(!s.contains("form "), "form block would require CLI args");
    }
}
