# 10 Ring-Mod Gale — notes

## Why this slot
Callback to track 06 Dorian Squall, but bigger: faster tempo, wider spectrum, and the four ring-mod carriers now sweep rather than hold. AABA form echoes track 02 of album 02 — the only form repetition across the corpus so far from a weather perspective.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem; narrow-band formant excitation at 400-800 Hz mirrors the carrier sweep range.
- Silero-VAD: audit voicing fraction ~0.05 ceiling (swept carriers can occasionally cross voiced-like regions).
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/ring_mod_multi.toml`. Variation `ring_mod_multi_gale_sweep`: all four `shift_addition_cc` stages take `--fifo` control and are swept linearly across the bar. Sweep ranges: carrier 1 from D3 (147) to A3 (220), carrier 2 from F3 (175) to C4 (262), carrier 3 from A3 (220) to E4 (330), carrier 4 from C4 (262) to G4 (392) — all dorian-aligned. Texture: wide-band `shaped_noise_bed` `-0.35 0.35` with `deemphasis_wfm_ff 48000 300e-6`. Sample source: `fir_cascade` for the high-band hiss of wind shear.

## Motif deployment
`m.room_impulse` at the end of each A section (bars 8, 12, 24) and at bar 28 as close. IR length 250 ms.

## Form (AABA, 28 bars of 4/4)
- A1 (1-8): carriers hold at low end.
- A2 (9-12): compressed A; carriers start sweeping.
- B (13-24): full carrier sweep sustained; widest spectrum.
- A3 (25-28): carriers settle back to low end; gale passes.

## openSMILE audit gates
- spectral_slope: -5 dB/oct +/- 2 dB.
- loudness integrated: -15 LUFS +/- 1 dB.
- band_limited_rms (300-3000 Hz): -16 dBFS.
- voicing_fraction: 0.05 ceiling.
