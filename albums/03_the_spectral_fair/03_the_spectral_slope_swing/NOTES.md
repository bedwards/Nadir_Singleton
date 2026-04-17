# The Spectral Slope Swing — notes

This is the album's title-adjacent dance, and its lyric spells out the two
spectral slope bands eGeMAPSv02 emits: `slope0-500_sma3` and
`slope500-1500_sma3`. The waltz meter inherits directly from
`m.feature_vector_waltz`; the swing feel is 60 % (closer to the jazz third).

## Slot rationale
Placing the slope song at track 3 sets brightness as the third axis of the
fair after pitch (01) and loudness (02). From here the listener has all three
perceptual primitives and the album can start combining families on later
attractions.

## Motif deployment
- Beat 1 = 0-500 Hz slope value (darker harmonic).
- Beat 2 = 500-1500 Hz slope value (brighter harmonic).
- Beat 3 = their difference (the "tilt").
- B section inverts the sign of both slopes, so the waltz leans the other way.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> bandpass 0-500 Hz   -> gain = beat1 slope -> busA
  -> bandpass 500-1500 Hz -> gain = beat2 slope -> busB
  mixer(busA, busB) -> ring_mod_multi -> bed
  granular_texture (carrier derived from tilt) -> texture
  mixer(vox, bed, texture) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "kilohertz" as `'k I l @ h 3:r ts` — final cluster kept light.
- "shallow" as `'S { l oU`, "slope" as `s l oU p` (no cluster issue).
- "secret" as `'s i k r @ t` — schwa before t, avoids plosive pile-up.

## openSMILE gates
- slope0-500 mean within plan +/- 8 dB/kHz.
- slope500-1500 mean within plan +/- 8 dB/kHz.
- F0 RMS error < 2 cents.
- Spectral flux mean delta < 10 % vs reference (keeps the swing honest).
