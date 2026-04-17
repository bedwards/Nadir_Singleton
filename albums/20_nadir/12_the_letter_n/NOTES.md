# 12 The Letter N — notes

## Descent step
Only the letter N (`-.`) remains. It is transmitted three times, separated by inter-letter 1400 ms gaps. After the noisy outgoing of 10 and the stammered reply of 11, the persona finds one clean letter and repeats it. This is the first initial of "Nadir" — the name the album is heading toward.

## Embedding of `m.nadir_silence`
Same grid. Entire piece is six units (three dashes, three dots) plus two 1400 ms gaps. Total track duration: roughly 3 × (600 + 200 + 200) + 2 × 1400 = 3000 + 2800 = 5800 ms of signal; pad head and tail with 1400 ms each for a ~8600 ms track at 40 bpm ≈ 6 bars. Manifest specifies 8 bars to allow tail silence to decay naturally.

## Reprises
- `m.singleton_spine` (album 19): album 19's recurring 12-note row collapses here to a single "note" — the letter N itself. The spine has one vertebra left.

## Pronunciation hints
- Dash-dot exactly 600 ms + 200 ms intra-gap + 200 ms aspirate = 1000 ms per N.
- Amplitude held constant at -18 dB RMS. No distancing — this N is close, certain, chosen.
- Optional room-tone bed at -42 dB (csdr shaped noise, lowpass 200 Hz) to certify the silences are not digital zero; document in `render.lock.toml` if applied.

## openSMILE gates
- voicing fraction: 0.0, ceiling 0.02.
- silence fraction: minimum 0.80.
- loudness integrated: -25.0 LUFS.
- Silero-VAD must detect exactly 6 onsets (3 × dash, 3 × dot). If not, re-render with stricter timing from the compose stage.
