# Loudness Barker — notes

We hand the loudness family its own attraction on track 2. Zwicker sone is
openSMILE's perceptual loudness LLD; this song converts that LLD into both
melody contour and gain envelope so the singer is literally performing the
functional.

## Slot rationale
Track 1 introduced F0 stats as the vertical axis of the fair. Track 2 gives us
the amplitude axis. Between them, every subsequent attraction can be placed on
a pitch x loudness grid. The barker persona is deliberate: loudness functionals
are the part of eGeMAPS most easily dramatised.

## Motif deployment
- 3/4 waltz, 138 bpm, swing 55 %.
- `m.feature_vector_waltz` beat assignment: beat 1 = loudness mean,
  beat 2 = loudness rising-slope, beat 3 = loudness peak.
- A sections crescendo bar by bar, A' decrescendo, B holds level (4 bars).

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate -> dsb_fc (loudness-follow gate)
  |-> ring_mod_multi (carriers 40 / 73 / 121 Hz) -> bed
  |-> granular_texture (grain gain = sone contour) -> texture
  '-> mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "louder" as `'l aU d @` — keep diphong open, slight length on `aU`.
- "nickel" as `'n I k @ l`, "holler" as `'h A l @`.
- Avoid clustering "perceptual" finals; syllabify as `p @ ' s E p tS u @ l`.

## openSMILE gates
- Loudness mean within +/- 0.5 sone of plan.
- Loudness peak - loudness mean <= 0.9 sone (prevents clipping theatrics).
- F0 RMS error < 2 cents.
- Integrated LUFS -13.5 +/- 0.4.
