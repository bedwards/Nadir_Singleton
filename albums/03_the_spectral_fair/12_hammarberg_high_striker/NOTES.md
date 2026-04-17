# Hammarberg High Striker — notes

`hammarbergIndex_sma3` is the ratio of the strongest peak in 0-2 kHz to the
strongest peak in 2-5 kHz — the classic brightness index used in voice
quality research. We pair it with the alpha-ratio track to finish the album's
brightness diptych.

## Slot rationale
Two brightness tracks in a row let the listener A/B them. Alpha ratio (11) is
band-energy-ratio; Hammarberg is peak-ratio. The physical contrast is carried
by different DSP shapes: alpha used broad bandpasses, Hammarberg uses narrow
peak pickers.

## Motif deployment
- 3/4 at 134 bpm, swing 60 %.
- Beat 1 = 0-2 kHz peak magnitude.
- Beat 2 = 2-5 kHz peak magnitude.
- Beat 3 = their ratio in dB.
- A sections climb the ratio; A' descends; B holds.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> FFT -> peak picker
    low_peak  (0-2 kHz) -> gain = beat1 -> low_peak_bus
    high_peak (2-5 kHz) -> gain = beat2 -> high_peak_bus
  mixer(low, high) -> ring_mod_multi -> bed
  granular_texture (grain pitch = ratio) -> texture
  mixer(vox, bed, texture) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "Hammarberg" as `'h { m @ b 3:r g` — keep consonants light.
- "kilohertz" as `'k I l @ h 3:r ts`.
- "lowland" as `'l oU l @ n d` — avoid cluster smear with schwa.

## openSMILE gates
- hammarbergIndex mean within +/- 2 dB of plan per bar.
- alphaRatio mean stable +/- 1 dB (we want Hammarberg to move independently).
- F0 RMS error < 2 cents.
- Loudness peak - mean <= 1.0 sone.
