# 02 Lessening — notes

## Descent step
A subtractive form. Each line is structurally shorter than the one before. Word count descends: 7, 4, 4, 4, 3, 3, 2, 1. Where line 8 is a single word "soon", the bar is padded by a 3200 ms silence — the silence is now more prominent than the syllable.

## Embedding of `m.nadir_silence`
We lengthen inter-phrase silences monotonically: 200 ms, 400 ms, 600 ms, 800 ms, 1000 ms, 1200 ms, 1400 ms. The final 1400 ms matches the Morse inter-letter gap used in track 14. We pre-train the listener's ear.

## Reprises
- `m.letter_cadence` (album 04): each line ends with its final syllable silence-padded before the next begins.
- `m.vowel_drone` (album 05): the sung "one" is a held `w V n`, the `V` stretched to 2.4 s.

## Pronunciation hints
- "thousand" → `T aU z @ n d`; soften terminal `d`.
- "soon" → `s u: n`; the held `u:` vowel is extended progressively on each repetition (0.4 s, 0.9 s, 1.8 s).
- No breath markings yet; we still breathe between lines normally.

## openSMILE gates
- voicing fraction: target 0.60, floor 0.50.
- pitch error RMS ceiling: 4.5 cents.
- loudness integrated: -18.5 LUFS.
- silence fraction (custom, computed via Silero-VAD `negative` frames): target 0.25 minimum — we require audible absence.
