# 13 Dirac Sleet — notes

## Why this slot
Penultimate track. Impulse rhythm returns (callback to 02 Patterned Drizzle and 08 Phase Hail) but at a mixed-temperature IR length that sits between drizzle's short tails and hail's hard metal. Sleet: icy, intermediate.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem — narrow single-formant at 2200 Hz (an icy resonance).
- Silero-VAD: audit voicing fraction ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/dirac_impulse_bed.toml`. Variation `dirac_impulse_bed_sleet_mixed_ir`: two parallel dirac-impulse chains summed — one with `peaks_fir_cc 512` (short, bright, hail-like) and one with `peaks_fir_cc 2048` (longer, damped, drizzle-like) — 6:4 mix in favour of the short chain, driven at 8th-note density. Texture: `granular_texture` with icy formant bandpass `0.04 0.06`. Sample source: `shaped_noise_bed` providing the wet noise floor beneath the impulses.

## Motif deployment
`m.room_impulse` is the track. Two IR-length modes deployed in parallel — the only album-06 track to mix IR lengths concurrently.

## Form (AB, 24 bars)
- A (bars 1-12): even short/long IR mix.
- B (bars 13-24): short IR chain thins to 4:6, long IR dominates, sleet softens into wet snow as the track closes.

## openSMILE audit gates
- spectral_slope: -5 dB/oct +/- 2 dB.
- loudness integrated: -17 LUFS +/- 1 dB.
- band_limited_rms (1k-8k Hz): -18 dBFS.
- voicing_fraction: 0.02 ceiling.
