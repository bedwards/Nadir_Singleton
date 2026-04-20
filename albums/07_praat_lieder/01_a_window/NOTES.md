# 01 A Window — notes

## Why this slot
We open the cycle on the smallest possible scene: a person standing near
glass before dawn. Nothing happens. The window is the subject; we are the
witness. This is the first time `m.whisper_psola` appears anywhere in the
corpus, so the track needs to *sound* like the motif being introduced —
MBROLA material, but the voicing pulled back by PSOLA until it is almost
breath.

## Form and motif deployment
Through-composed, 24 bars in F# minor at 54 bpm, 3/4. The short triple
meter gives each line an unhurried, almost-spoken lilt. We introduce
`m.whisper_psola` across the whole track: every vocal phrase is MBROLA
us1 output re-pitched via PSOLA with a reduced range factor (0.75) so
the contour flattens toward spoken intonation, and voicing amplitude is
shaved by post-processing in Praat before resynthesis.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts`. Pitch
extraction: autocorrelation (`ac`), floor 65 Hz, ceiling 1000 Hz,
very-accurate on. Target median 220 Hz. Range factor 0.75 — we compress
the F0 range around the median so every syllable tends toward the same
hushed pitch. Formant shift ratio 1.00 (we do not alter vocal-tract
character here; the whisper comes from the pitch flattening, not from
formant manipulation).

## csdr graph shape
`band_limit` preset at -24 dB under the vocal, band-limited 200–2200 Hz
so the bed sits strictly below the formants and does not smear the
sibilants. Several bars in the middle run pure silence (bed gated off
bars 9–14). No reverb.

## G2P / pronunciation hints
- "window" as /'w I n d oU/ — clean, do not nasalise the /n d/ join.
- "pane" as /p eI n/ — short, held /n/.
- "breath" as /b r E T/ — the final theta must be audible but not fricated.
- "agree" as /@ 'g r i/ — schwa first, then a clean /g r i/.

## openSMILE gate
Voicing fraction on the vocal stem in [0.35, 0.55] — *low*, because the
whisper motif deliberately reduces voiced energy. Pitch-error RMS 2 cents
ceiling. Loudness gated at -18 LUFS integrated (quieter than the corpus
norm; this is an art-song cycle).
