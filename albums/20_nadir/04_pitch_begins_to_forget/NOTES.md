# 04 Pitch Begins to Forget — notes

## Descent step
Scale membership dissolves. The compose-stage PSOLA target is now `chromatic` (all 12 pitches), and the snap tolerance is widened to 15 cents. Realised F0 will wander off the grid by design; the pitch-error gate is deliberately loose so that Praat does not overcorrect. The Nadir persona begins to forget tonality.

## Embedding of `m.nadir_silence`
Every third bar holds a 1400 ms silence. The number three matters — it is the count used in tracks 6 and 7 to further erode form.

## Reprises
- `m.quarter_tone_approach` (album 10): each phrase approaches its target tone via a 50-cent neighbour that never resolves.
- `m.modal_shift` (album 16): across the 14 bars the scale slowly rotates A minor → A Phrygian → A Locrian → unnamed. No single scale holds.

## Pronunciation hints
- "minor" → `m aI n @`; nasal `n` held 280 ms, pitch slurs through 40 cents during the hold.
- "between" → `b I t w i: n`; three repetitions in bars 6–8, each a semitone lower than the last.
- "forget" → `f @ g E t`; final `t` unreleased, bleeds to silence.

## openSMILE gates
- voicing fraction: target 0.45, floor 0.35.
- pitch error RMS ceiling: 15.0 cents (deliberately loose).
- loudness integrated: -19.5 LUFS.
- F0 stddev semitones: we invert the usual gate — minimum 1.5 ST stddev required (we want wobble).
