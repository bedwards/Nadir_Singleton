# 11 FFT Mist — notes

## Why this slot
Penultimate-side track; after the gale, mist settles. We use csdr's FFT blocks to make the mist explicitly spectral — brushing a narrow comb across the band so individual bins become audible as slow spectral shimmer.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem with F1 = 500 Hz and slow bandwidth modulation providing a moisture-like spectral shimmer.
- Silero-VAD: audit voicing fraction ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output; the only track on album 06 where `fft_cc` is part of the primary chain.

## csdr graph shape
Primary = `presets/granular_texture.toml`. Variation `granular_texture_mist_slow`: replace the granular's short 20 ms gating window with 400 ms, so instead of grains we get slow mist breaths. Precede with `fft_cc 1024 4 HAMMING` → `peaks_fir_cc 1024 <10 carrier rates comb>` → `fft_fc 1024 4` for explicit FFT-domain comb filtering. Texture: `fir_cascade` with high-band emphasis (`0.3 0.48`). Sample source: sparse `dirac_impulse_bed`.

## Motif deployment
`m.room_impulse` sparse — two deployments, bars 8 and 20, IR length 500 ms each, behaving as spectral "clearings" through the mist.

## openSMILE audit gates
- spectral_slope: -7 dB/oct +/- 2 dB.
- loudness integrated: -21 LUFS +/- 1 dB.
- band_limited_rms (1k-6k Hz): -23 dBFS.
- voicing_fraction: 0.02 ceiling.
- spectral_flux: should read low (mist is slow); if above the track-09 reference multiplier, regen with slower granular window.
