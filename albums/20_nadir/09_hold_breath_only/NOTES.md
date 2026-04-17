# 09 Hold (Breath Only) — notes

## Descent step
Phonation is withdrawn. The track is inhalations and exhalations — unvoiced aspiration only. MBROLA us3 emits `_ 800` (silence) interleaved with `h 600` (aspirate) segments. No pitch contour is specified; `tessitura_hz = [0, 0]` is a sentinel meaning "no F0 target". Praat PSOLA is bypassed on this track; the voice has stepped aside.

## Embedding of `m.nadir_silence`
The whole track is silence-and-breath. We adopt the track 14 timings as rehearsal:
- `[hh]` = 600 ms aspirate (Morse dash analogue)
- `[h]`  = 200 ms aspirate (Morse dot analogue)
- `.`   = 1400 ms pure silence (inter-letter gap)

The pattern is: 4 dashes, 4 dots, 1 final dash. This is not a Morse letter — it is a pre-echo of the structural units that track 14 will assemble into "NADIR".

## Reprises
- `m.whisper_psola` (album 07): we extend the whisper tradition to its logical zero — from half-voiced to fully aspirated. The Praat script `psola_retarget.praat` is skipped; we run `psola_devoice.praat` with gain -inf (i.e. all spectral energy moved above the source).

## Pronunciation hints
- MBROLA `.pho` skeleton (approx): `_ 400 ; h 600 ; _ 1400 ; h 600 ; _ 1400 ; ...`
- No lyric words — the G2P stage is bypassed. `lyric.txt` uses bracket notation `[hh]` / `[h]` as directives for the render pipeline.
- Breath intensity envelope: each `[hh]` crescendos from 0 to -18 dB across 600 ms; each `[h]` is flat at -22 dB.

## openSMILE gates
- voicing fraction: target 0.0, ceiling 0.05. If voicing > 5 percent, re-render with F0 fully unset.
- pitch error: NOT APPLICABLE (no pitch target). We skip the gate.
- loudness integrated: -24.0 LUFS.
- spectral flatness (`spectralFlatness_sma3_amean`): floor 0.35 — we need the noise-like signature of breath.
- openSMILE still required in `render.lock.toml` to certify the absence of voicing.
