# 07 Granular Snow — notes

## Why this slot
Side A closes. We revisit snow, this time granular — the flakes are audible as distinct grains, not a smooth wash. Follows `research/csdr.md` granular-texture recipe: tens of parallel bandpass branches with per-branch gating at 20-80 ms windows, summed.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem — F1/F2/F3 centered at 400/1800/3400, each grain-windowed at 30 ms to mimic whisper-breath consonant noise bursts.
- Silero-VAD: audit voicing fraction ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/granular_texture.toml`. Variation `granular_texture_snow_20branches`: run the preset 20 times in parallel with bandpass carrier rates distributed log-uniformly from 200 Hz to 6 kHz (`shift_addition_cc` rates 0.0042 through 0.125), each branch gated via `gain_ff --fifo` on a 40 ms on / 60 ms off pattern with per-branch phase offset. Sum outside csdr in `nadir-dsp`'s mix op. Texture: `shaped_noise_bed` with airy bandpass `0.05 0.45`. Sample source: sparse `dirac_impulse_bed` adding one impulse every 8 bars — a distant pine creak.

## Motif deployment
`m.room_impulse` very sparse: four deployments across 32 bars, IR length 400 ms, positioned to mark phrase entries.

## openSMILE audit gates
- spectral_slope: -4 dB/oct +/- 2 dB (pink-ish but not as deep as track 04).
- loudness integrated: -20 LUFS +/- 1 dB.
- band_limited_rms (200-6000 Hz): -22 dBFS +/- 2 dB (the granular zone).
- voicing_fraction: 0.03 ceiling.
