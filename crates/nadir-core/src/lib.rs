//! nadir-core: shared data model for the Nadir_Singleton production system.
//!
//! The five primitive tools (openSMILE, Praat, MBROLA, Silero-VAD, csdr) are the
//! only things that touch audio; the types in this crate describe the intent that
//! drives them: scores, phonemes, pitch contours, rhythms, stems, tracks, albums.

pub mod album;
pub mod pho;
pub mod pitch;
pub mod score;
pub mod stem;
pub mod track;

pub use album::{Album, AlbumId};
pub use pho::{Pho, PhoStream, PitchPoint};
pub use pitch::{Key, Note, PitchHz, Scale, ScaleKind};
pub use score::{Form, Meter, Part, Score};
pub use stem::{RenderLock, Stem, StemKind};
pub use track::{Track, TrackN};

pub const SAMPLE_RATE: u32 = 48_000;
pub const MBROLA_SAMPLE_RATE: u32 = 16_000;
pub const VAD_SAMPLE_RATE: u32 = 16_000;
