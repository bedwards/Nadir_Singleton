# HNR Tent — notes

The harmonics-to-noise ratio is a quiet family, so this attraction is the
album's first hush. We drop key to E minor and loudness target to -14.5 LUFS;
the listener walks inside a tent and the midway recedes.

## Slot rationale
Placing HNR at track 4 lets the carnival breathe after three bright outdoor
attractions. It also sets up a contrast needed later: jitter (05) and shimmer
(06) will re-introduce noise on purpose. Here we still honour clean harmonics.

## Motif deployment
- `m.feature_vector_waltz` 3/4 still in force but at 120 bpm and 55 % swing.
- Beat 1 = HNR mean. Beat 2 = HNR minimum. Beat 3 = HNR slope.
- `m.whisper_psola` from album 07's palette is foreshadowed for 8 bars in the B
  section: Praat PSOLA de-voices the MBROLA output to drop HNR deliberately.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> Praat psola (de-voice 50 % on B) -> vox
  |-> ring_mod_multi (low carrier 28 Hz) -> bed (subtle)
  |-> granular_texture (grain_len = 80 ms, breathy tail) -> texture
  '-> mixer (vox 0.8, bed 0.1, texture 0.1) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "harmony" as `'h A r m @ n i` — keep the final i open.
- "shadows" as `'S { d oU z` — soft final z.
- "narrow" as `'n { r oU` — no r-cluster issue.

## openSMILE gates
- HNR mean > 12 dB on A sections.
- HNR mean 6-10 dB on B section (designed drop).
- Jitter local < 1.5 % (we do not want jitter bleed here).
- F0 RMS error < 2 cents.
