use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Write};

/// A pitch anchor inside a phoneme, expressed as (percent of duration, Hz).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PitchPoint {
    pub pct: u8,
    pub hz: f32,
}

/// A single MBROLA phoneme directive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pho {
    pub sampa: String,
    pub dur_ms: u32,
    pub pitch: Vec<PitchPoint>,
}

impl Pho {
    pub fn silence(dur_ms: u32) -> Self {
        Self {
            sampa: "_".into(),
            dur_ms,
            pitch: vec![],
        }
    }

    pub fn voiced(sampa: impl Into<String>, dur_ms: u32, hz: f32) -> Self {
        Self {
            sampa: sampa.into(),
            dur_ms,
            pitch: vec![PitchPoint { pct: 0, hz }, PitchPoint { pct: 100, hz }],
        }
    }

    /// Emit a single `.pho` line.
    pub fn to_line(&self, buf: &mut String) {
        write!(buf, "{} {}", self.sampa, self.dur_ms).unwrap();
        for p in &self.pitch {
            write!(buf, " {} {:.2}", p.pct, p.hz).unwrap();
        }
        buf.push('\n');
    }
}

/// Ordered phoneme stream ready for MBROLA stdin.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoStream {
    pub items: Vec<Pho>,
}

impl PhoStream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, p: Pho) -> &mut Self {
        self.items.push(p);
        self
    }

    pub fn total_ms(&self) -> u32 {
        self.items.iter().map(|p| p.dur_ms).sum()
    }
}

impl Display for PhoStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in &self.items {
            let mut buf = String::new();
            p.to_line(&mut buf);
            f.write_str(&buf)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_line() {
        let mut s = String::new();
        Pho::silence(100).to_line(&mut s);
        assert_eq!(s, "_ 100\n");
    }

    #[test]
    fn voiced_line() {
        let mut s = String::new();
        Pho::voiced("a", 200, 220.0).to_line(&mut s);
        assert!(s.contains("a 200"));
        assert!(s.contains("220.00"));
    }

    #[test]
    fn stream_join() {
        let mut st = PhoStream::new();
        st.push(Pho::silence(50));
        st.push(Pho::voiced("n", 80, 220.0));
        let s = st.to_string();
        assert_eq!(s.lines().count(), 2);
        assert_eq!(st.total_ms(), 130);
    }
}
