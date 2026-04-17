# 03 Low-Shelf Rain — notes

## Why this slot
Rain has arrived. We commit to a steady, continuous wash with a deep low-shelf dominating the spectrum. This is the first "big weather" on the album.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem as sample-rate sync anchor.
- Praat: KlattGrid texture stem; formants parked at 300/1000/2800 Hz to mimic the resonance of a rain-wet alley.
- Silero-VAD: audit voicing fraction ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/deemphasis_chain.toml`. Variation `deemphasis_chain_rain_low`: two cascaded `deemphasis_wfm_ff 48000 150e-6` stages feeding a narrow-low `bandpass_fir_fft_cc -0.1 0.01 0.005` to emphasise the sub-kHz shelf. Bed: `shaped_noise_bed` with wider bandpass (`-0.45 0.45`) to keep high-band hiss barely present. Sample source: `dirac_impulse_bed` at high density (30-40 hits/bar) with short 30 ms IRs, emulating distinct droplets on a hard surface.

## Motif deployment
`m.room_impulse` deployed at maximum density so far: the rain IS the impulse bed, not just accented by it. IR length 30 ms; decay buried under the shelf.

## Form
AB: A bars 1-12 steady rain, B bars 13-28 the shelf softens (`deemphasis_wfm_ff` tau halves) and the impulse density thins by 40% — the rain is letting up.

## openSMILE audit gates
- spectral_slope: -12 dB/oct +/- 2 dB (this is the steepest slope on the album).
- loudness integrated: -16 LUFS +/- 1 dB.
- band_limited_rms (80-500 Hz): -14 dBFS +/- 2 dB (low-shelf band hot).
- voicing_fraction: 0.02 ceiling.
