# 09 Hemlock Fog — notes

## Why this slot
After hail, the world mutes. We hold a long near-static fog bank. 40 bars at 40 BPM = 4 minutes — the album's longest track. Centre of side B, the listener needs decompression after track 08.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem anchoring a long uninterrupted section; important for sample-rate stability across 4 minutes.
- Praat: KlattGrid texture stem — wide-bandwidth vowel drone near A at 110 Hz, -40 dB, providing near-inaudible formant colour to the fog.
- Silero-VAD: audit voicing fraction ~0; the KlattGrid drone is quiet enough that VAD must still report ~0.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/shaped_noise_bed.toml`. Variation `shaped_noise_bed_fog_broad`: broad bandpass `-0.3 0.3`, `deemphasis_wfm_ff 48000 800e-6` for a warm-mid dominant spectrum, and a very slow LFO-driven `gain_ff --fifo` (0.05 Hz amplitude wobble) to make the fog breathe. Texture: dense-lowmid `deemphasis_chain`. Sample source: `dirac_impulse_bed` with very long IR (`peaks_fir_cc 8192` — ~170 ms tail) for distant foghorn-like bloom.

## Motif deployment
`m.room_impulse` deployed three times at bars 10, 22, 33 — each with 1.5 s IR tail, evoking a foghorn in an estuary. This is the single-longest IR deployment on album 06.

## openSMILE audit gates
- spectral_slope: -8 dB/oct +/- 2 dB.
- loudness integrated: -23 LUFS +/- 1 dB (second quietest after 01).
- band_limited_rms (200-1500 Hz): -24 dBFS +/- 2 dB.
- voicing_fraction: 0.02 ceiling.
- LUFS-short peak spread across 40 bars must not exceed 2 dB — if it does, the fog is "breathing" too visibly.
