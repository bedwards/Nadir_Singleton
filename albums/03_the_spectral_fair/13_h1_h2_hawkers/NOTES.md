# H1-H2 Hawkers — notes

`logRelF0-H1-H2_sma3nz` captures the dB difference between the first and
second harmonic — a classic voice-quality indicator, with high H1-H2
signalling breathy voice and low/negative signalling pressed voice. We cast
two hawkers at rival stalls, one breathy, one pressed.

## Slot rationale
Harmonic-difference features sit late in eGeMAPS, and their dramatic contrast
(breathy vs pressed) wants a 6/8 sway for shoulder-to-shoulder calling. Slot
13 is the last 6/8 of the album before the final three 3/4 tracks.

## Motif deployment
- 6/8 at 128 bpm, swing 55 %.
- First half-bar: beats 1-3 = H1, H2, H1-H2 on the breathy side.
- Second half-bar: beats 4-6 = H1, H2, H1-H2 on the pressed side.
- us3 voice, more overtone content, shows the difference better.

## csdr graph shape
```
MBROLA vox (us3) -> convert_s16_f -> fir_interpolate
  -> Praat PSOLA (slightly de-voice for breathy passes)
  -> harmonic picker (FFT -> f0 track -> H1, H2 bins)
    H1 gain = beat1/4 target
    H2 gain = beat2/5 target
  mixer(H1, H2) -> ring_mod_multi -> bed
  granular_texture -> texture
  mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "hawker" as `'h O k @` — keep vowel open.
- "breathy" as `'b r E T i` — soft final th.
- Avoid "difference" cluster: syllabify `'d I f @ r @ n s`.

## openSMILE gates
- logRelF0-H1-H2 mean breathy side: 4-8 dB.
- logRelF0-H1-H2 mean pressed side: -2 to 2 dB.
- HNR mean > 9 dB (floor).
- F0 RMS error < 2 cents.
