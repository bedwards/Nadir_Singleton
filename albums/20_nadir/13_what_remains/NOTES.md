# 13 What Remains — notes

## Descent step
One last spoken word: "we". Three times, each separated by a single long aspirated dash (`[hh]`, 600 ms). After track 12 transmitted the letter, track 13 names the transmitter. This is the last appearance of phonation in the corpus — after this, only silence and breath.

## Embedding of `m.nadir_silence`
Between each "we" and each `[hh]`: 1400 ms pure silence. The structure reads as: `we _1400_ [hh] _1400_ we _1400_ [hh] _1400_ we`. Five 1400 ms gaps, three `we`s, two aspirates. The silences dominate 7 : 1 by duration.

## Reprises
- `m.vowel_drone` (album 05): the `i:` of each "we" is held 1.6 s, the longest drone of the album. Pitch stable at 110 Hz ± 1 Hz.
- `m.drift_delta` (album 12): across the three "we"s the held pitch drifts from 112 Hz → 110 Hz → 108 Hz. Two-Hz steps, -30 cents total drift across the track. The corpus's pitch-crawl motif delivers its final drift here.

## Pronunciation hints
- "we" → `w i:`; duration 1.6 s; F0 held on target per line above.
- `[hh]` → us3 phoneme `h` at 600 ms duration, -18 dB RMS.
- Breath in and out natural; no false-start attacks.

## openSMILE gates
- voicing fraction: target 0.25 (three short voiced islands in majority silence), floor 0.18.
- pitch error RMS ceiling: 30.0 cents (our loosest; we honour the decay).
- loudness integrated: -23.0 LUFS (briefly louder than surrounding silent tracks — the last word).
- pitch must be logged: the three F0 centroids are stored to `render.lock.toml` to verify the -30 cent drift.
