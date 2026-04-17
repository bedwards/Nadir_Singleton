# 06 We Count by Threes — notes

## Descent step
Meter drops to 3/4. Only beat 1 carries a syllable; beats 2 and 3 are silence. The `.` in `lyric.txt` represents an intentional empty beat (one inter-onset of ~700 ms at 42 bpm per beat in triple time). The pulse is thinning; we say one word per bar instead of many.

## Embedding of `m.nadir_silence`
Every empty beat is a 700 ms silence — between the 200 ms Morse dot and the 600 ms dash. We are priming the listener to parse silences as units. The 1400 ms inter-letter Morse gap appears at bar 4, 8, 12 transitions.

## Reprises
- `m.feature_vector_waltz` (album 03): the 3/4 is explicit homage — but where 03 had one eGeMAPS family per beat, here each non-tonic beat has zero energy.
- `m.onset_volley` (album 14): Silero-VAD will find exactly one onset per bar. We run VAD at threshold 0.3 to confirm; log any beat with detected voicing.

## Pronunciation hints
- All single syllables; keep SAMPA minimal. "still" → `s t I l`, held 240 ms.
- On repeats of "three", decay pitch 30 cents each iteration (A3 → ~ 170 Hz slipping).
- No breath between syllables — the silence is the rest itself.

## openSMILE gates
- voicing fraction: target 0.25, floor 0.18 (we are now majority-silence).
- pitch error RMS ceiling: 20.0 cents.
- loudness integrated: -20.5 LUFS.
- silence fraction (Silero-VAD negative): minimum 0.65 — the gate reverses here; we must be mostly silent.
