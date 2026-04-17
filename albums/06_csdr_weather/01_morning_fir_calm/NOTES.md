# 01 Morning FIR Calm — notes

## Why this slot
We open album 06 with near-silence: the listener steps outside at the edge of dawn, before a weather system arrives. This is the calibration point for every other track — the quietest spectral slope on the album, the sparsest `m.room_impulse` deployment.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""` means no vocal part, but the five-tool render lock is still honoured:
- MBROLA: still invoked to emit a zero-amplitude silence stem as sample-rate sync anchor. A four-bar `_` pause `.pho` renders to 16 kHz silence, upsampled via csdr `rational_resampler_ff 3 1` to the 48 kHz master clock.
- Praat: runs `klatt_grid_texture.praat` to emit one neutral KlattGrid bed at -30 dB; it mixes under the csdr stack as a formant-coloured hiss.
- Silero-VAD: audits the silence stem. Expected voicing fraction ~0.0 (sanity check that the zero-amp anchor actually contains no voice).
- openSMILE: feature audit only — spectral_slope, loudness, band_limited_rms. No voicing/pitch gates because there is no vocal.
- csdr: the primary output. Everything audible is csdr.

## csdr graph shape
Primary output = `presets/fir_cascade.toml` with cascaded bandpasses shrunk by half (variation `fir_cascade_calm_soft`: `0.005-0.05`, `0.05-0.15`, `0.15-0.2`, all at transition_bw `0.003`). Bed: `presets/shaped_noise_bed.toml` variation narrowed to `-0.08 0.08` for a sub-kHz hush. Sample source: sparse `dirac_impulse_bed` with IR length extended via a long `peaks_fir_cc 4096` tail to approximate a cold outdoor room impulse.

## Motif deployment
`m.room_impulse` shows up exactly three times across 32 bars, IR length 800 ms each, decaying into the noise bed. These are the only articulated events; everything else is continuous bed.

## openSMILE audit gates
- spectral_slope: target -9 dB/oct +/- 2 dB.
- loudness integrated: -24 LUFS +/- 1 dB (softest track on the album).
- band_limited_rms (80-3200 Hz): below -30 dBFS.
- voicing_fraction: 0.02 ceiling; over that we regen with deeper FIR attenuation.
