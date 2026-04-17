use serde::{Deserialize, Serialize};

pub type PitchHz = f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Key {
    pub fn semitone(self) -> i32 {
        self as i32
    }
    pub fn base_hz(self) -> f32 {
        440.0 * 2f32.powf((self.semitone() - 9) as f32 / 12.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScaleKind {
    Major,
    Minor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    Chromatic,
    PentatonicMajor,
    PentatonicMinor,
    Custom(Vec<u8>),
}

impl ScaleKind {
    pub fn intervals(&self) -> Vec<u8> {
        match self {
            ScaleKind::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleKind::Minor => vec![0, 2, 3, 5, 7, 8, 10],
            ScaleKind::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11],
            ScaleKind::MelodicMinor => vec![0, 2, 3, 5, 7, 9, 11],
            ScaleKind::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            ScaleKind::Phrygian => vec![0, 1, 3, 5, 7, 8, 10],
            ScaleKind::Lydian => vec![0, 2, 4, 6, 7, 9, 11],
            ScaleKind::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
            ScaleKind::Locrian => vec![0, 1, 3, 5, 6, 8, 10],
            ScaleKind::Chromatic => (0..12).collect(),
            ScaleKind::PentatonicMajor => vec![0, 2, 4, 7, 9],
            ScaleKind::PentatonicMinor => vec![0, 3, 5, 7, 10],
            ScaleKind::Custom(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    pub key: Key,
    pub kind: ScaleKind,
}

impl Scale {
    pub fn new(key: Key, kind: ScaleKind) -> Self {
        Self { key, kind }
    }

    /// Snap a frequency to the nearest scale degree within 4 octaves around A4.
    pub fn snap(&self, hz: PitchHz) -> PitchHz {
        let root_semi = self.key.semitone();
        let ivals = self.kind.intervals();
        let mut best = hz;
        let mut best_err = f32::INFINITY;
        for octave in -2..=5 {
            for iv in &ivals {
                let semi = root_semi + (*iv as i32) + 12 * octave;
                let f = 440.0 * 2f32.powf((semi - 9) as f32 / 12.0);
                let err = (hz.ln() - f.ln()).abs();
                if err < best_err {
                    best_err = err;
                    best = f;
                }
            }
        }
        best
    }

    pub fn degrees_hz(&self, octave: i32) -> Vec<PitchHz> {
        let root_semi = self.key.semitone() + 12 * octave;
        self.kind
            .intervals()
            .iter()
            .map(|iv| 440.0 * 2f32.powf((root_semi + *iv as i32 - 9) as f32 / 12.0))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub hz: PitchHz,
    pub dur_ms: u32,
    pub syllable: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_major_hz() {
        assert!((Key::A.base_hz() - 440.0).abs() < 0.01);
    }

    #[test]
    fn snap_to_scale() {
        let s = Scale::new(Key::A, ScaleKind::Minor);
        let snapped = s.snap(441.0);
        assert!((snapped - 440.0).abs() < 0.5);
    }

    #[test]
    fn minor_intervals() {
        assert_eq!(ScaleKind::Minor.intervals(), vec![0, 2, 3, 5, 7, 8, 10]);
    }
}
