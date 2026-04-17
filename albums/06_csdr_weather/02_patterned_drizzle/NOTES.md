# 02 Patterned Drizzle — notes

## Why this slot
Second track; we introduce rhythmic impulses. Calm morning now gets its first drops — 16th-note pattern, randomised amplitude, held sparse enough to feel like drizzle and not rain.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem, 24 bars at 96 BPM = 60 s of 16 kHz silence, upsampled through csdr.
- Praat: one KlattGrid texture stem acting as the drizzle's vowel-colour ground.
- Silero-VAD: audit voicing fraction ~0 on the silence anchor stem.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/dirac_impulse_bed.toml`. Variation `dirac_impulse_bed_drizzle_16th`: the `peaks_fir_cc` IR length stays at 1024 but the driving `yes_f` is gated on a 16th-note pattern; we key the gate through `gain_ff --fifo` from a 96 BPM pulse (24 hits/s window averaging ~4 hits/bar once stochastic drop is applied). Bed: `shaped_noise_bed` narrowed to mid-band (`-0.1 0.1`). Sample source: `fir_cascade` acting as a drip-to-drip tail shaper.

## Motif deployment
`m.room_impulse` governs the entire track. IR length 120 ms (shorter than 01). Density ramps from 3 hits/bar (bars 1-8) to 9 hits/bar (bars 17-24), then resets to 3 for the final bar.

## openSMILE audit gates
- spectral_slope: -6 dB/oct +/- 2 dB.
- loudness integrated: -18 LUFS +/- 1 dB.
- band_limited_rms (2-8 kHz): -22 dBFS +/- 2 dB (drizzle lives in the mid-highs).
- voicing_fraction: 0.02 ceiling.
