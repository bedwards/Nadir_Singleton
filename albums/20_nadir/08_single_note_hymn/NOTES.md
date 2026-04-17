# 08 Single-Note Hymn — notes

## Descent step
Melody collapses to a single pitch: A2 at 110 Hz. The `custom` scale is `[0]` — one tone. Every syllable lands on the tonic. What was melody in earlier albums is now memory; the hymn mourns the octave by refusing it. Tessitura window in `manifest.toml` is deliberately narrow (105–115 Hz) to enforce the one-pitch constraint.

## Embedding of `m.nadir_silence`
Between each line we insert one Morse-dash-length (600 ms) silence. Between each stanza, one Morse-inter-letter (1400 ms). Lines 9–12 — the four "we"s — are separated by precisely the 1400 ms gap. Four "we"s, four gaps = eight calibrated units.

## Reprises
- `m.vowel_drone` (album 05): the final "we" is held 6 s, `i:` vowel stable at 110 Hz. This is where the vowel-drone tradition of the corpus resolves.
- `m.fir_blessing` (album 15): vocal is passed through the original album-15 cascaded bandpass chain, unchanged.
- `m.hook` (album 11, 18): the repeated "we hold the one" is a 4-bar refrain — the last real hook of the corpus.

## Pronunciation hints
- "one" → `w V n`; sustained 1.2 s on each occurrence.
- "hold" → `h @U l d`; diphthong `@U` drifts from 111 Hz to 109 Hz — micro `m.drift_delta`.
- "we" → `w i:`; increasingly airy across the four closing repetitions (voicing fraction 0.9 → 0.5).

## openSMILE gates
- voicing fraction: target 0.50 (back up — the track is sung), floor 0.40.
- pitch error RMS ceiling: 25.0 cents.
- loudness integrated: -21.5 LUFS.
- F0 stddev semitones: maximum 0.3 ST — this is our monotone gate. If the voice wanders, we re-PSOLA.
