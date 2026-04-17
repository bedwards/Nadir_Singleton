//! nadir-compose: scale-snapped melody and rhythm generation for vocals.

use nadir_core::{Note, Pho, PhoStream, PitchPoint, Scale};

/// Assign scale-snapped pitches to a sequence of syllables.
/// `stresses` maps each syllable to a duration weight (1.2 = primary, 0.85 = unstressed, 1.0 = neutral).
/// `bpm` drives the base note duration: a beat is 60000/bpm ms; stressed syllables get a full beat,
/// unstressed get half a beat.
pub fn plan_melody(
    scale: &Scale,
    syllables: &[String],
    seed: u64,
    center_hz: f32,
    bpm: f32,
    stresses: &[f32],
) -> Vec<Note> {
    let beat_ms = (60000.0 / bpm.max(1.0)) as u32;
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
    for (i, s) in syllables.iter().enumerate() {
        idx = (idx + step()).clamp(0, degrees.len() as i32 - 1);
        let stress = stresses.get(i).copied().unwrap_or(1.0);
        // primary stress (≥1.2) → full beat, unstressed (<0.9) → half beat, else 3/4
        let dur_ms = if stress >= 1.15 {
            beat_ms
        } else if stress < 0.9 {
            (beat_ms / 2).max(80)
        } else {
            (beat_ms * 3 / 4).max(100)
        };
        notes.push(Note {
            hz: degrees[idx as usize],
            dur_ms,
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
        let notes = plan_melody(&scale, &syls, 42, 220.0, 96.0, &[1.2, 0.85, 1.0]);
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
    fn bpm_drives_duration() {
        let scale = Scale::new(Key::A, ScaleKind::Minor);
        let syls = vec!["na".to_string(), "dir".to_string()];
        // primary stressed (1.2) at 120 bpm = 500ms beat
        let notes = plan_melody(&scale, &syls, 1, 220.0, 120.0, &[1.2, 0.85]);
        assert_eq!(notes[0].dur_ms, 500, "primary stress → full beat at 120 bpm");
        assert_eq!(notes[1].dur_ms, 250, "unstressed → half beat at 120 bpm");
    }

    #[test]
    fn pho_stream_has_content() {
        let scale = Scale::new(Key::A, ScaleKind::Minor);
        let syls = vec!["na".to_string(), "dir".to_string()];
        let notes = plan_melody(&scale, &syls, 1, 220.0, 96.0, &[1.0, 1.0]);
        let phs = vec![
            vec!["n".to_string(), "a".to_string()],
            vec!["d".to_string(), "i".to_string(), "r".to_string()],
        ];
        let s = render_vox_pho(&notes, &phs);
        assert!(s.total_ms() > 300);
    }
}
