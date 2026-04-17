# Shimmer Carousel — notes

Shimmer (`shimmerLocaldB_sma3nz`) is jitter's amplitude cousin. Where track 05
wobbled pitch, here we wobble level. The carousel metaphor makes the wobble
periodic on purpose: each 3-beat bar traces one rise and one fall.

## Slot rationale
Jitter + shimmer deserve back-to-back tracks so the listener hears the pair
symmetry; slot 06 lets the album's middle third do micro-instability proper
before moving on to higher-order features in 07-09.

## Motif deployment
- 3/4, 130 bpm, light swing.
- Beat 1 = shimmer mean, beat 2 = shimmer stddev, beat 3 = shimmer peak.
- Between A and A' we slowly raise the shimmer target (0.3 dB to 0.7 dB).
- B holds it level — the carousel steadies for one tour.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> dsb_fc (amplitude modulator, 0.5 Hz carrier) -> vox (shimmered)
  |-> ring_mod_multi (organ-pipe carriers 65, 98, 131 Hz) -> bed
  |-> granular_texture (grain gain from shimmer LLD) -> texture
  '-> mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "shimmer" as `'S I m @`, "saddle" as `'s { d @ l`.
- "carousel" as `k { r u 's E l` — final l soft.
- "painted" as `'p eI n t @ d` — schwa before the d avoids cluster.

## openSMILE gates
- Shimmer local 0.3-0.9 dB (planned amplitude spread).
- Jitter local < 1.0 % (we separate the families).
- F0 RMS error < 2 cents.
- Loudness peak - loudness mean <= 1.0 sone.
