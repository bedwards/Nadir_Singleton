# 05 Band-Limit Thunder — notes

## Why this slot
Middle of side A. We introduce the album's first climactic event: thunder. To keep it synthetic and DSP-only, we bandlimit noise aggressively to sub-200 Hz and gate it with long-tail dirac impulses — no sampled thunder.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem as sample-rate anchor.
- Praat: KlattGrid texture stem parked at a single F1 = 120 Hz to reinforce the rumble band from a different synthesis path.
- Silero-VAD: audit voicing fraction ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms (critical: 20-200 Hz band RMS dominates the mix).
- csdr: the primary output.

## csdr graph shape
Primary = `presets/shaped_noise_bed.toml`. Variation `shaped_noise_bed_sub_200`: bandpass tightened to `-0.004 0.004` (i.e. effectively a 192 Hz brickwall lowpass at 48 kHz), `deemphasis_wfm_ff 48000 2000e-6` for extra warm thump, then `gain_ff 0.8`. Texture: `dirac_impulse_bed` with `peaks_fir_cc 4096` for long-tail IR (reverb-like low rumble continuation). Sample source: `fir_cascade` trimmed to single low bandpass.

## Motif deployment
`m.room_impulse` with explicitly long IR length (2-3 s). Four impulses across 20 bars, spaced unevenly — bars 2, 7, 13, 18. Each is a thunder-crack envelope.

## openSMILE audit gates
- spectral_slope: -15 dB/oct +/- 2 dB (extreme low-skew).
- loudness integrated: -14 LUFS +/- 1 dB (loudest track on side A).
- band_limited_rms (20-200 Hz): -10 dBFS +/- 2 dB.
- band_limited_rms (2k-8k): below -40 dBFS (hard gate — any mid-high content means the bandpass slipped).
- voicing_fraction: 0.02 ceiling.
