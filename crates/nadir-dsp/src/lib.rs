//! nadir-dsp: construct and execute csdr pipelines.
//!
//! A `Graph` is a list of `Stage`s. Each stage is a csdr command with its args.
//! `Graph::run_piped` spawns the chain as a Unix pipeline, reading from an input
//! file (or stdin) and writing to an output file (or stdout).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stage {
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl Stage {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self {
            cmd: cmd.into(),
            args: vec![],
        }
    }
    pub fn arg(mut self, a: impl Into<String>) -> Self {
        self.args.push(a.into());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    /// Input sample rate of the first stage's input stream.
    pub in_sr: u32,
    /// Output sample rate expected from the last stage.
    pub out_sr: u32,
    /// Ordered stages.
    pub stages: Vec<Stage>,
    /// Path to the `csdr` binary (default: "csdr" on PATH).
    #[serde(default = "default_bin")]
    pub bin: String,
}

fn default_bin() -> String {
    "csdr".into()
}

impl Graph {
    pub fn parse_toml(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }

    /// Describe the pipeline as a shell string (for debug/logging, not for exec).
    pub fn to_shell(&self) -> String {
        self.stages
            .iter()
            .map(|s| {
                let mut parts = vec![self.bin.clone(), s.cmd.clone()];
                parts.extend(s.args.iter().cloned());
                parts.join(" ")
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }

    /// Run the pipeline: raw bytes from `input` piped through every stage, result
    /// written to `output` (both are filesystem paths). Assumes the csdr build
    /// offers every stage as `csdr <cmd> [args...]`.
    pub fn run_files(&self, input: &Path, output: &Path) -> Result<()> {
        if self.stages.is_empty() {
            anyhow::bail!("empty graph");
        }
        tracing::info!(pipeline = %self.to_shell(), "csdr run");
        let in_file = fs_err::File::open(input).with_context(|| format!("open {input:?}"))?;

        let mut children: Vec<Child> = Vec::with_capacity(self.stages.len());
        let mut prev_stdout: Option<std::process::ChildStdout> = None;
        for (i, stage) in self.stages.iter().enumerate() {
            let mut cmd = Command::new(&self.bin);
            cmd.arg(&stage.cmd).args(&stage.args);
            cmd.stdin(if i == 0 {
                Stdio::from(in_file.try_clone()?.into_parts().0)
            } else {
                Stdio::from(prev_stdout.take().expect("prev stdout"))
            });
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            let mut child = cmd
                .spawn()
                .with_context(|| format!("spawn stage {}: {} {:?}", i, stage.cmd, stage.args))?;
            prev_stdout = child.stdout.take();
            children.push(child);
        }

        let mut last_stdout = prev_stdout.expect("no last stdout");
        let mut out_file = fs_err::File::create(output)?;
        let mut buf = [0u8; 65536];
        loop {
            let n = last_stdout.read(&mut buf)?;
            if n == 0 {
                break;
            }
            out_file.write_all(&buf[..n])?;
        }

        for (i, mut c) in children.into_iter().enumerate() {
            let status = c.wait()?;
            if !status.success() {
                let mut err = String::new();
                if let Some(mut se) = c.stderr {
                    let _ = se.read_to_string(&mut err);
                }
                anyhow::bail!("csdr stage {} failed ({}): {}", i, status, err);
            }
        }
        Ok(())
    }
}

pub mod presets;
