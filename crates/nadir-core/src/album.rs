use crate::track::Track;
use serde::{Deserialize, Serialize};

pub type AlbumId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: AlbumId,
    pub n: u8,
    pub slug: String,
    pub title: String,
    pub narrative: String,
    pub tracks: Vec<Track>,
}
