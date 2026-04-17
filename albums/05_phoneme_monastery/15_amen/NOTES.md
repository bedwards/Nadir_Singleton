# 15 Amen — notes

## Why this slot
The close of the album. We return to matins' /a/ at matins' tempo
(40 bpm) — the monastic day is a ring. The word "amen" is the only
articulated word, and it is sung deliberately as two vowels
(`a men` → `a` + `m E n`) so that the /a/ of the album is embedded
inside the /m E n/ of the closing word. `m.vowel_drone` carried
throughout; `m.letter_cadence` on the final "men"; a last reprise of
`m.dawn_utterance` inside the closing gesture because the album ends
where it began. The track also seeds `m.vowel_drone` forward into
album 19, per CORPUS.md: "m.vowel_drone — … returns in 19."

## Form and motif deployment
Through-composed, 18 bars at 40 bpm. Structure:
- Bars 1–6: four /a/ drones (7–9 s) at A3, progressively quieter
  (-18, -19, -20, -21 LUFS each).
- Bars 7–11: "a men" sung three times at A3–C4–A3 — the C4 on the `E`
  of `men` is the `m.dawn_utterance` fragment (rising minor third),
  now used as cadence not opening.
- Bars 12–14: one bare /a/ (8 s).
- Bars 15–17: final "a men" descending A3–G3–F3 — the brotherhood
  kneels.
- Bar 18: one last /a/ at A3, held 10 s and faded by 9 dB over the
  final 4 s. The silence after is the actual end of the album and is
  intentional — not trimmed in mastering.

## csdr graph shape
`fir_cascade` tuned to /a/ F1/F2 (720/1240 Hz) throughout.
`dirac_impulse_bed` at 0.5 Hz — matins' tempo reprised. At bar 17 the
impulse bed cuts, leaving the voice bare for its final drone. The last
2 s are pure silence except for a fade-residual of the voice.

## G2P / pronunciation hints
- `a` → SAMPA `a`; durations 7–9 s in bars 1–6, 10 s in final bar.
- "a men" → `a m E n`; break the word into two clear parts: `a` held
  1.2 s, then `m E n` sung as a single syllable 1.8 s with `n`
  sustained.
- The third "a men" (bar 11) rises `a` (A3) → `m E n` (C4) — the
  `m.dawn_utterance` fragment.
- The final "a men" (bars 15–17) descends `a` (A3) → `m E` (G3) →
  `n` (F3); each pitch held 1.5 s. The descending minor third is the
  mirror of the opening minor third — the album's formal rhyme.

## openSMILE gates
- voicing fraction: target 0.70, floor 0.60.
- pitch error RMS ceiling: 2.0 cents (the tightest on the album —
  amen is precise).
- loudness integrated: -21.0 LUFS.
- F0 stddev semitones across any held vowel: maximum 0.3 ST.
- silence fraction (Silero-VAD): final 2 s must measure as complete
  silence (voicing fraction 0).
