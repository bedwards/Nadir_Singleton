//! nadir-compose: scale-snapped melody and rhythm generation for vocals.

use nadir_core::{Note, Pho, PhoStream, PitchPoint, Scale};

/// Assign scale-snapped pitches to a sequence of syllables.
/// `stresses` maps each syllable to a duration weight (1.2 = primary, 0.85 = unstressed, 1.0 = neutral).
/// `bpm` drives the base note duration. Equivalent to `plan_melody_phrased` with
/// a single phrase covering every syllable.
pub fn plan_melody(
    scale: &Scale,
    syllables: &[String],
    seed: u64,
    center_hz: f32,
    bpm: f32,
    stresses: &[f32],
) -> Vec<Note> {
    plan_melody_phrased(scale, syllables, &[syllables.len()], seed, center_hz, bpm, stresses)
}

/// Phrase-shaped melody. `phrase_lens[i]` is the number of syllables in phrase i
/// (must sum to syllables.len()). Each phrase gets a deterministic contour
/// (arc-up, arc-down, ascent, descent, return) chosen from the seed + phrase index.
pub fn plan_melody_phrased(
    scale: &Scale,
    syllables: &[String],
    phrase_lens: &[usize],
    seed: u64,
    center_hz: f32,
    bpm: f32,
    stresses: &[f32],
) -> Vec<Note> {
    let beat_ms = (60000.0 / bpm.max(1.0)) as u32;
    let degrees = scale.degrees_hz(0);
    if degrees.is_empty() {
        return vec![];
    }
    let center_idx = degrees
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
    // Range in scale-degree units for phrase contour amplitude.
    let range: i32 = 3;

    // LCG for jitter
    let mut rng_state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut jitter = || -> i32 {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        match (rng_state >> 32) as u32 % 5 {
            0 => -1,
            1 | 2 | 3 => 0,
            _ => 1,
        }
    };

    let mut notes = Vec::with_capacity(syllables.len());
    let mut syl_cursor = 0usize;
    let max_degree = degrees.len() as i32 - 1;

    for (p, &plen) in phrase_lens.iter().enumerate() {
        if plen == 0 {
            continue;
        }
        let contour_kind = ((seed.wrapping_add(p as u64 * 0x9E3779B9)) >> 40) % 5;
        for k in 0..plen {
            let i = syl_cursor + k;
            let u = if plen == 1 { 0.5 } else { k as f32 / (plen - 1) as f32 };
            let offset_f = match contour_kind {
                0 => (std::f32::consts::PI * u).sin() * range as f32,          // arc-up
                1 => -(std::f32::consts::PI * u).sin() * range as f32,         // arc-down
                2 => (u * 2.0 - 1.0) * range as f32,                            // ascent
                3 => (1.0 - u * 2.0) * range as f32,                            // descent
                _ => ((std::f32::consts::PI * 2.0 * u).sin() * 0.5) * range as f32, // return
            };
            let stress = stresses.get(i).copied().unwrap_or(1.0);
            // Stress boost: primary-stressed syllables nudge toward contour peak
            let stress_boost = if stress >= 1.15 { 1 } else { 0 };
            let idx = (center_idx + offset_f.round() as i32 + jitter() + stress_boost)
                .clamp(0, max_degree);
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
                syllable: syllables[i].clone(),
            });
        }
        syl_cursor += plen;
    }
    notes
}

/// Convert (syllable,phoneme-list,dur,hz) into a PhoStream with pitch anchors.
/// Single-phrase shim for callers that haven't adopted phrase-aware breaths.
pub fn render_vox_pho(notes: &[Note], phonemes_per_syl: &[Vec<String>]) -> PhoStream {
    render_vox_pho_phrased(notes, phonemes_per_syl, &[notes.len()], 30, 400)
}

/// Phrase-aware variant: `phrase_lens[i]` = number of syllables in phrase i.
/// Between syllables inside a phrase → `intra_ms` silence. At phrase boundaries
/// (and after the last phrase) → `breath_ms` silence. Opening/closing silence
/// = `breath_ms`. Portamento glides the first phoneme of each in-phrase syllable
/// from the previous note's pitch to the current, giving a legato line.
pub fn render_vox_pho_phrased(
    notes: &[Note],
    phonemes_per_syl: &[Vec<String>],
    phrase_lens: &[usize],
    intra_ms: u32,
    breath_ms: u32,
) -> PhoStream {
    let mut stream = PhoStream::new();
    stream.push(Pho::silence(breath_ms));
    let mut cursor = 0usize;
    for (p_idx, &plen) in phrase_lens.iter().enumerate() {
        for k in 0..plen {
            let i = cursor + k;
            if i >= notes.len() { break; }
            let n = &notes[i];
            let phs = &phonemes_per_syl[i];
            let total: u32 = n.dur_ms;
            let per = (total / phs.len().max(1) as u32).max(40);
            // Previous note's pitch — only within the phrase (phrase boundaries reset)
            let prev_hz = if k == 0 {
                None
            } else {
                notes.get(i - 1).map(|p| p.hz)
            };
            for (j, p) in phs.iter().enumerate() {
                let is_last = j + 1 == phs.len();
                let is_first = j == 0;
                let dur = if is_last {
                    total - per * (phs.len() as u32 - 1)
                } else {
                    per
                };
                // First phoneme of an in-phrase syllable starts at previous pitch
                // and slides to current (10%→90%); others hold current pitch.
                let (start_hz, end_hz) = match (is_first, prev_hz) {
                    (true, Some(prev)) => (prev, n.hz),
                    _ => (n.hz, n.hz),
                };
                stream.push(Pho {
                    sampa: p.clone(),
                    dur_ms: dur,
                    pitch: vec![
                        PitchPoint { pct: 10, hz: start_hz },
                        PitchPoint { pct: 90, hz: end_hz },
                    ],
                });
            }
            // Silence between syllables within a phrase
            if k + 1 < plen {
                stream.push(Pho::silence(intra_ms));
            }
        }
        cursor += plen;
        let _is_last_phrase = p_idx + 1 == phrase_lens.len();
        stream.push(Pho::silence(breath_ms));
    }
    stream
}

#[cfg(test)]
mod tests {
    use super::*;
    use nadir_core::{Key, ScaleKind};

    #[test]
    fn phrase_contour_lengths() {
        let scale = Scale::new(Key::A, ScaleKind::Minor);
        let syls: Vec<String> = "a b c d e f".split_whitespace().map(str::to_string).collect();
        let notes = plan_melody_phrased(&scale, &syls, &[2, 4], 7, 220.0, 96.0, &[1.0; 6]);
        assert_eq!(notes.len(), 6);
        for n in &notes {
            let snapped = scale.snap(n.hz);
            assert!((n.hz - snapped).abs() < 0.001);
        }
    }

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
