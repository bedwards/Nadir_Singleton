# 04 Pink Snow — notes

## Why this slot
The rain has quieted. We approximate pink noise (-3 dB/oct) to evoke falling snow: broadband but warm, no high-frequency sparkle. This is the album's first true equal-loudness-contour ambient moment.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem. 36 bars at 44 BPM = ~196 s.
- Praat: KlattGrid texture stem; formants tuned to approximate a hollow-room vowel colour buried at -35 dB.
- Silero-VAD: audit voicing fraction ~0 on the silence anchor.
- openSMILE: spectral_slope, loudness, band_limited_rms — pink slope is the critical target here.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/shaped_noise_bed.toml`. Variation `shaped_noise_bed_pink`: drop the tight `-0.2 0.2` bandpass to `-0.45 0.45` (full-band), then cascade two `deemphasis_wfm_ff 48000 500e-6` stages instead of one to reach the -3 dB/oct pink slope documented in `research/csdr.md`. Texture: `deemphasis_chain` with soft rolloff. Sample source: very sparse `dirac_impulse_bed` — one impulse every 4-8 bars acting as a distant branch snap under the wash.

## Motif deployment
`m.room_impulse` deployed rarely, contrasting 03 Low-Shelf Rain's density. IR length 600 ms; each click dies into the pink wash.

## openSMILE audit gates
- spectral_slope: -3 dB/oct +/- 1 dB (this is the pink-noise gate; out-of-tolerance triggers regen with deemphasis tau adjustment).
- loudness integrated: -22 LUFS +/- 1 dB.
- band_limited_rms (broadband 20-20000 Hz): -20 dBFS.
- voicing_fraction: 0.02 ceiling.
