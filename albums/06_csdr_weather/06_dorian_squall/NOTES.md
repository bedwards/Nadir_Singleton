# 06 Dorian Squall — notes

## Why this slot
The weather becomes musical. We tune the four ring-mod carriers to dorian scale ratios so the wind resolves into a faint modal pitch-field. This is the only track on album 06 with an explicit key/scale beyond ambient washes.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem with F1/F2 locked to D-dorian degrees (D 147, F 175, G 196) so the Praat path also speaks the mode.
- Silero-VAD: audit voicing fraction ~0 (slightly higher ceiling 0.05 here because modal ring-mod can fool the VAD on formant-adjacent bands).
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/ring_mod_multi.toml`. Variation `ring_mod_multi_dorian_carriers`: four `shift_addition_cc` carriers retuned to `147/48000`, `175/48000`, `220/48000`, `262/48000` Hz (D3, F3, A3, C4 — four dorian tones). Texture: `fir_cascade` with the high band raised to `0.4 0.48` to emulate wind's hiss peak. Sample source: `shaped_noise_bed` providing the noise that rides the ring-mod carriers.

## Motif deployment
`m.room_impulse` deployed at the ends of A and A' sections: one IR-tail click to demarcate form — bar 8, bar 24, and bar 32. IR length 200 ms.

## Form (6/8, 32 bars)
- A (bars 1-8): carriers at D/F.
- B (bars 9-24): all four carriers engaged; texture bandpass sweeps.
- A' (bars 25-32): carriers drop back to D/F; squall abates.

## openSMILE audit gates
- spectral_slope: -6 dB/oct +/- 2 dB.
- loudness integrated: -15 LUFS +/- 1 dB.
- band_limited_rms (500-4000 Hz): -16 dBFS (the wind band).
- voicing_fraction: 0.05 ceiling.
