use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StemKind {
    Vox,
    Bed,
    Texture,
    Sample,
    Mix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stem {
    pub kind: StemKind,
    pub path: String,
    pub gain_db: f32,
    pub pan: f32,
}

/// Reproducibility record for a track render.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RenderLock {
    pub nadir_version: String,
    pub mbrola_version: String,
    pub praat_version: String,
    pub opensmile_version: String,
    pub silero_version: String,
    pub csdr_version: String,
    pub invocations: Vec<Invocation>,
    pub rng_seed: u64,
    pub input_hashes: Vec<(String, String)>,
    pub started_at: String,
    pub finished_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invocation {
    pub tool: String,
    pub argv: Vec<String>,
    pub stdin_sha256: Option<String>,
    pub stdout_sha256: Option<String>,
    pub exit_code: i32,
    pub wall_ms: u64,
}

impl RenderLock {
    pub fn touched_tools(&self) -> std::collections::BTreeSet<String> {
        self.invocations.iter().map(|i| i.tool.clone()).collect()
    }

    /// Returns `Ok(())` when all five core tools appear in the invocation list.
    pub fn verify_all_five_tools(&self) -> anyhow::Result<()> {
        let touched = self.touched_tools();
        let required = ["mbrola", "praat", "SMILExtract", "silero-vad", "csdr"];
        for t in required {
            if !touched.iter().any(|x| x.as_str() == t) {
                anyhow::bail!("core tool {t} not used in this render");
            }
        }
        Ok(())
    }
}
