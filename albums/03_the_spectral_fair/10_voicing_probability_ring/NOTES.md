# Voicing Probability Ring — notes

`voicingFinalUnclipped_sma3nz` is the posterior voicing probability, in 0..1,
gate for every `_nz` functional. We dramatise it as the high-striker bell:
each bar hits the lever and the bell rings according to the voicing fraction
of that bar.

## Slot rationale
Voicing is the gatekeeper for half of eGeMAPS, so placing its own track at 10
clarifies why earlier tracks (01, 07) were allowed to reference pitch at all.
It also sets up a contrast with the next two brightness tracks (11 alpha,
12 Hammarberg) which are unvoiced-friendly.

## Motif deployment
- 3/4 at 132 bpm, swing 58 %.
- Beat 1 = voicing fraction for the bar, beat 2 = longest voiced run,
  beat 3 = unvoiced fraction.
- Downbeats ring the "bell" in DSP when voicing > 0.8; the ring is a Praat
  sine tuned to the tonic A4.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> silero-vad -> voicing mask
  |-> praat sine A4 gated by (mask > 0.8) -> bell_bus
  |-> ring_mod_multi -> bed
  |-> granular_texture (grains on voiced frames only) -> texture
  mixer(vox, bell, bed, texture) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "voicing" as `'v OI s I N` — final velar nasal.
- "probability" as `p r A b @ 'b I l @ t i` — light syllables throughout.
- "hammer" as `'h { m @`, "lever" as `'l i v @`.

## openSMILE gates
- Voicing fraction > 0.75 in A sections, < 0.55 in B.
- voicingFinalUnclipped mean > 0.6.
- F0 RMS error < 2 cents (voiced frames only).
- HNR mean > 10 dB.
