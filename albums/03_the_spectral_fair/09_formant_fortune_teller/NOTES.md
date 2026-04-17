# Formant Fortune Teller — notes

Three formant frequencies (F1, F2, F3) each land on a card. The 6/8 meter here
is intentional so that two formants fit per half-bar and the third lands on a
weak beat — a card reveal.

## Slot rationale
Formants are the biggest remaining family and deserve a pair of tracks. This
is the frequency partner; bandwidth gets track 14. Slot 09 sits deep in the
album so the listener's ear is already trained to segment functionals.

## Motif deployment
- 6/8 at 126 bpm, swing 55 % (waltz variant per CORPUS.md).
- First half-bar: beats 1-3 = F1, F2, F3 frequencies.
- Second half-bar: beats 4-6 = F1, F2, F3 one step up the scale.
- us3 voice gives richer mid formants for clearer card-reveal contrast.

## csdr graph shape
```
MBROLA vox (us3) -> convert_s16_f -> fir_interpolate
  -> Praat formant shift script (F1 +/- 50 Hz, F2 +/- 120 Hz per card)
  |-> ring_mod_multi (carriers derived from current F1) -> bed
  |-> granular_texture (grain pitch = F3 target) -> texture
  '-> mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "formant" as `'f O r m @ n t` — open final schwa.
- "resonance" as `'r E z @ n @ n s` — light final s.
- "frequencies" as `'f r i k w @ n s i z` — avoid cluster on f-r.

## openSMILE gates
- F1 frequency mean within +/- 40 Hz of plan.
- F2 frequency mean within +/- 80 Hz of plan.
- F3 frequency mean within +/- 120 Hz of plan.
- F0 RMS error < 2 cents.
