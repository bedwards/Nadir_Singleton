use crate::score::Score;
use crate::stem::Stem;
use serde::{Deserialize, Serialize};

pub type TrackN = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub n: TrackN,
    pub slug: String,
    pub title: String,
    pub score: Score,
    pub stems: Vec<Stem>,
    pub mix_path: Option<String>,
    pub narrative_role: String,
}
