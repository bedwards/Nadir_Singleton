//! nadir-render: orchestrates the five tools into a track render.
//!
//! Phase 1 (this crate) wires invocations and produces a `RenderLock`. Phase 2
//! (future PRs) adds the actual audio bus mixing via csdr, pitch audit loop,
//! and quality gates.

use anyhow::Result;
use nadir_core::{RenderLock, Score};
use std::path::Path;

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
    // Stubbed: concrete pipeline assembly arrives in #render-pipeline issue.
    let lock = RenderLock::default();
    Ok(lock)
}
