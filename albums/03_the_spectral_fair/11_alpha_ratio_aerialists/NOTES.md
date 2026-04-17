# Alpha Ratio Aerialists — notes

`alphaRatio_sma3` is the energy ratio between the 50-1000 Hz band and the
1-5 kHz band — a lumped brightness. Aerialists on a tightrope dramatise the
balance: when the ratio is low, the low-band performer hangs heavy; when it
rises, the high-band performer lifts.

## Slot rationale
Alpha ratio belongs next to the other brightness measure (Hammarberg, track
12), so the pair sits in the back third of the album. Placing it before
Hammarberg matches the order in eGeMAPSv02's feature list.

## Motif deployment
- 3/4 at 136 bpm, swing 58 %.
- Beat 1 = low-band energy, beat 2 = high-band energy, beat 3 = ratio.
- A sections: ratio climbs bar by bar. A' descends. B holds ratio constant.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> splitter
    -> bandpass 50-1000 Hz  -> gain = low-band beat -> low_bus
    -> bandpass 1000-5000 Hz -> gain = high-band beat -> high_bus
  mixer(low, high) -> ring_mod_multi -> bed
  granular_texture (grain brightness = ratio) -> texture
  mixer(vox, bed, texture) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "aerialist" as `E r i @ l I s t` — no cluster issue with schwas.
- "kilohertz" as `'k I l @ h 3:r ts` — kept light; final ts is fine.
- "ribbon" as `'r I b @ n`, "rafters" as `'r { f t @ z`.

## openSMILE gates
- alphaRatio mean within +/- 2 dB of plan per bar.
- F0 RMS error < 2 cents.
- Loudness mean within +/- 0.5 sone.
- Spectral flux mean delta < 10 %.
