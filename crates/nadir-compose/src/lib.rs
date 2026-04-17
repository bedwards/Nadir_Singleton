//! nadir-compose: scale-snapped melody and rhythm generation for vocals.

use nadir_core::{Note, Pho, PhoStream, PitchPoint, Scale};

/// Assign scale-snapped pitches to a sequence of syllables. The contour is a
/// simple stepwise random-walk bounded to an octave around the mid-tessitura.
pub fn plan_melody(scale: &Scale, syllables: &[String], seed: u64, center_hz: f32) -> Vec<Note> {
    let mut rng_state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut step = || -> i32 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((rng_state >> 32) as i32 % 3) - 1
    };
    let degrees = scale.degrees_hz(0);
    if degrees.is_empty() {
        return vec![];
    }
    let mut idx = degrees
        .iter()
        .enumerate()
        .min_by(|a, b| {
            (a.1.ln() - center_hz.ln())
                .abs()
                .partial_cmp(&(b.1.ln() - center_hz.ln()).abs())
                .unwrap()
        })
        .map(|(i, _)| i as i32)
        .unwrap_or(0);
    let mut notes = Vec::with_capacity(syllables.len());
    for s in syllables {
        idx = (idx + step()).clamp(0, degrees.len() as i32 - 1);
        notes.push(Note {
            hz: degrees[idx as usize],
            dur_ms: 350,
            syllable: s.clone(),
        });
    }
    notes
}

/// Convert (syllable,phoneme-list,dur,hz) into a PhoStream with pitch anchors.
pub fn render_vox_pho(notes: &[Note], phonemes_per_syl: &[Vec<String>]) -> PhoStream {
    let mut stream = PhoStream::new();
    stream.push(Pho::silence(120));
    for (n, phs) in notes.iter().zip(phonemes_per_syl) {
        let total: u32 = n.dur_ms;
        let per = (total / phs.len().max(1) as u32).max(40);
        for (i, p) in phs.iter().enumerate() {
            let is_last = i + 1 == phs.len();
            let dur = if is_last {
                total - per * (phs.len() as u32 - 1)
            } else {
                per
            };
            stream.push(Pho {
                sampa: p.clone(),
                dur_ms: dur,
                pitch: vec![
                    PitchPoint { pct: 10, hz: n.hz },
                    PitchPoint { pct: 90, hz: n.hz },
                ],
            });
        }
        stream.push(Pho::silence(30));
    }
    stream.push(Pho::silence(120));
    stream
}

#[cfg(test)]
mod tests {
    use super::*;
    use nadir_core::{Key, ScaleKind};

    #[test]
    fn melody_stays_in_scale() {
        let scale = Scale::new(Key::A, ScaleKind::Minor);
        let syls = vec!["na".to_string(), "dir".to_string(), "sin".to_string()];
        let notes = plan_melody(&scale, &syls, 42, 220.0);
        for n in &notes {
            let snapped = scale.snap(n.hz);
            let cents = 1200.0 * (n.hz / snapped).ln() / std::f32::consts::LN_2;
            assert!(
                cents.abs() < 0.1,
                "note {} not on scale ({}¢ off {})",
                n.hz,
                cents,
                snapped
            );
        }
    }

    #[test]
    fn pho_stream_has_content() {
        let scale = Scale::new(Key::A, ScaleKind::Minor);
        let syls = vec!["na".to_string(), "dir".to_string()];
        let notes = plan_melody(&scale, &syls, 1, 220.0);
        let phs = vec![
            vec!["n".to_string(), "a".to_string()],
            vec!["d".to_string(), "i".to_string(), "r".to_string()],
        ];
        let s = render_vox_pho(&notes, &phs);
        assert!(s.total_ms() > 300);
    }
}
