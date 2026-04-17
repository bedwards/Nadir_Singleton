# 12 Deemphasis Dusk — notes

## Why this slot
The weather system retires. We run a deep deemphasis chain to darken the spectrum as if the air itself has dimmed with light. Pair to track 03 Low-Shelf Rain — both use `deemphasis_chain` as primary, but 03 emphasises the shelf, 12 emphasises the slope.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem with slowly lowering F1 from 700 Hz to 300 Hz across 32 bars — a "vowel darkening" under the csdr deemphasis.
- Silero-VAD: audit voicing fraction ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/deemphasis_chain.toml`. Variation `deemphasis_chain_dusk_deep`: three cascaded `deemphasis_wfm_ff 48000 800e-6` stages then a hard `bandpass_fir_fft_cc -0.06 0.04 0.003` finish, yielding an extremely soft spectrum ~-10 dB/oct. Texture: `shaped_noise_bed` with warm-mid bandpass `-0.15 0.15` and an extra `deemphasis_wfm_ff 48000 500e-6`. Sample source: `dirac_impulse_bed` very sparse with long IR — the final impulses of album 06 ring here.

## Motif deployment
`m.room_impulse` placed at bars 1, 12, 22, 32 — each IR length 1.0 s, decaying into the darker spectrum, echoing the album's opener.

## openSMILE audit gates
- spectral_slope: -10 dB/oct +/- 2 dB.
- loudness integrated: -21 LUFS +/- 1 dB.
- band_limited_rms (2k-8k Hz): below -34 dBFS (hard gate; any mid-high content means deemphasis slipped).
- voicing_fraction: 0.02 ceiling.
