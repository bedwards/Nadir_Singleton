use crate::pitch::{Note, Scale};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Meter(pub u8, pub u8);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Form {
    AABA,
    VerseChorus,
    ThroughComposed,
    Binary,
    Ternary,
    Rondo,
    Strophic,
    Custom(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Part {
    Vox(VoxPart),
    Bed(DspPart),
    Texture(DspPart),
    Sample(PraatPart),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoxPart {
    pub lyric: String,
    pub mbrola_voice: String,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspPart {
    pub name: String,
    pub graph_path: String,
    pub gain_db: f32,
    pub pan: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PraatPart {
    pub script: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub title: String,
    pub scale: Scale,
    pub bpm: f32,
    pub meter: Meter,
    pub bars: u32,
    pub form: Form,
    pub parts: Vec<Part>,
}
