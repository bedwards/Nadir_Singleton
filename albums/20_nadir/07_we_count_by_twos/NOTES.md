# 07 We Count by Twos — notes

## Descent step
Meter collapses to 2/4: on-off, on-off. Every syllable is followed by exactly one beat of silence (750 ms at 40 bpm in duple). The lyric is stripped to the pattern itself — "on", "off", and the pronoun "we". The song is its own metronome.

## Embedding of `m.nadir_silence`
The 750 ms silences are nearly the 800 ms dash width of the lengthening Morse sequence. The listener, having traversed tracks 2–6, now parses silence as rhythm. We place a 1400 ms inter-letter gap every 4 bars.

## Reprises
- `m.plosive_rhythm` (album 17): "on" and "off" lean on the plosive `n`-stop and `f`-fricative for percussion; no extra csdr percussion layer.
- `m.room_impulse` (album 06, 14): between "on" and "off" we drop a single Dirac-click through a 90 ms IR — a room-shaped tick, -18 dB.

## Pronunciation hints
- "on" → `O n`; strongly released `n` with nasal tail 120 ms.
- "off" → `O f`; `f` unvoiced, 220 ms, extended into the silence.
- "we" → `w i:`; the `i:` held 400 ms on each repetition, pitch stable at 110 Hz.

## openSMILE gates
- voicing fraction: target 0.20, floor 0.14.
- pitch error RMS ceiling: 22.0 cents.
- loudness integrated: -21.0 LUFS.
- shimmer localdB: allowed up to 3x album-01 baseline — voice is now ghosting.
