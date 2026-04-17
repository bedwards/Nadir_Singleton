# Jitter Hall of Mirrors — notes

Jitter is a cycle-to-cycle F0 perturbation; we dramatise it by compounding the
3/4 into a 6/8 at 144 bpm, so every bar now folds two triplets into one — the
listener hears the waltz reflected against itself.

## Slot rationale
After the hush of the HNR tent, we need motion. Jitter is the only eGeMAPS
family that is, by definition, a micro-instability; placing it at the album's
midpoint opening lets the second half start with sanctioned wobble.

## Motif deployment
- 6/8 is an acknowledged variant of `m.feature_vector_waltz` (note from
  CORPUS.md: occasional 6/8). Beat 1,3,5 = jitter mean. Beat 2,4,6 = jitter
  stddev.
- us3 voice (richer mid formants) is used here because jitter is more audible
  on a rounder source. The rest of the album returns to us1.

## csdr graph shape
```
MBROLA vox (us3) -> convert_s16_f -> fir_interpolate
  -> Praat manipulation (introduce +/- 0.6 % period deviation)
  |-> shift_addition_cc (delay 2 ms) -> mirror_bus_A
  |-> shift_addition_cc (delay 4 ms) -> mirror_bus_B
  mixer(vox, A, B) -> ring_mod_multi -> bed
  granular_texture (grain len 12 ms, jittered offsets) -> texture
  mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "mirror" as `'m I r @` — keep the schwa light.
- "jitter" as `'dZ I t @`, "wobble" as `'w A b @ l`.
- Avoid clustering "trembling"; syllabify as `' t r E m b l I N`.

## openSMILE gates
- Jitter local 2-4 % (designed; higher than the usual ceiling).
- F0 RMS error < 3 cents (loosened ceiling is deliberate here).
- HNR mean > 8 dB (floor so it does not descend into noise).
- Shimmer local < 0.5 dB (prevent amplitude coupling with the jitter theatre).
