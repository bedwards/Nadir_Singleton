# Spectral Flux Flume — notes

Spectral flux (`spectralFlux_sma3`) measures frame-to-frame change in the
spectrum; every onset is a spike. We treat the LLD contour as the shape of a
log-flume ride: gentle rises, sudden splashes at consonant onsets.

## Slot rationale
Track 08 takes the flux family solo because it is the best onset proxy in
eGeMAPSv02 and naturally pairs with the VAD rhythm engine. The placement here
hands the listener a faster waltz (140 bpm) before the album's formant half.

## Motif deployment
- 3/4, 140 bpm, 52 % swing.
- Beat 1 = flux mean, beat 2 = flux rising-slope, beat 3 = flux peak.
- Every bar seeds one "splash" at a VAD-detected onset: Silero-VAD on the vox
  stem emits onset times, which trigger Praat-generated Dirac impulses routed
  through csdr FIR shaping.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  |-> silero-vad -> onsets.json
  |-> praat dirac clicks @ onsets -> fir (bandpass 800-3200 Hz) -> splash_bus
  |-> ring_mod_multi -> bed
  |-> granular_texture (grain trigger = flux peak) -> texture
  mixer(vox, splash, bed, texture) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "flume" as `f l u m` — long vowel, no cluster issue.
- "ratio" as `'r eI S i oU`, "delta" as `'d E l t @`.
- Avoid "attacks" cluster: use `@ ' t { k s` with a tiny schwa.

## openSMILE gates
- Spectral flux mean within plan +/- 10 %.
- VAD onset count matches plan +/- 2 per 4-bar phrase.
- F0 RMS error < 2 cents.
- Loudness peak - mean < 1.1 sone (splashes allowed a little theatre).
