# 08 Phase Hail — notes

## Why this slot
Side B opens with the album's most aggressive track. Hail on corrugated metal: hard transients, metallic ring, bright spectrum, fast tempo. The drizzle of track 02 has become weaponised.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem.
- Praat: KlattGrid texture stem; F3/F4 at 3500/4500 Hz for metallic formant colour barely audible under the csdr mix.
- Silero-VAD: audit voicing fraction ~0 on the silence anchor.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/dirac_impulse_bed.toml`. Variation `dirac_impulse_bed_hail_32nd`: `peaks_fir_cc 256` (short IR, bright), driving impulses at 32nd-note density = 16 hits/bar at 132 BPM = ~35 hits/second. Texture: `ring_mod_multi` with carriers retuned to inharmonic metallic ratios (713, 1089, 1543, 2211 Hz — chosen to avoid consonant pitch pull). Sample source: `fir_cascade` shaping the impulse tail to peak at 3-5 kHz.

## Motif deployment
`m.room_impulse` deployed at maximum density on the album. IR length 40 ms per hit; high density creates a metallic curtain. Density ramps down bars 15-20 as the hail passes.

## openSMILE audit gates
- spectral_slope: -2 dB/oct +/- 1 dB (brightest slope on album; metallic content dominates).
- loudness integrated: -13 LUFS +/- 1 dB (loudest track on album).
- band_limited_rms (2k-8k Hz): -12 dBFS (hot mid-highs).
- band_limited_rms (20-200 Hz): below -30 dBFS (we keep the lows out).
- voicing_fraction: 0.02 ceiling.
