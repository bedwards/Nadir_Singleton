# 01 First Descent — notes

## Descent step
We open the coda with full lyrical content intact, but tuning already slips. `m.drift_delta` (0.2 cents/bar) is doubled to 0.4 cents/bar here and the slope is strictly downward — pitch crawls flat as the album opens. The vocal is still continuous; silence is present only as inter-phrase breath.

## Embedding of `m.nadir_silence`
At the four end-of-line positions we insert a single calibrated silence of 1400 ms (the inter-letter gap width used in track 14). This is a seed — listeners do not yet parse it as Morse, but the clock has started.

## Reprises
- `m.vowel_drone` (album 05): the held `a` on "air" in line 4 is 6 s, micro-vibrato damped to zero by bar 16.
- `m.dawn_utterance` (album 01): the opening "we begin" quotes the original rising-3rd-then-plateau contour, but pitched a minor 6th below the dawn statement. The corpus inverts.

## Pronunciation hints
- "salted" → `s O l t I d` (MBROLA us3 SAMPA); stress the `O`, shorten final `I d`.
- "descend" → `d I s E n d`; final `d` is unreleased, bleeding into the track-end silence.
- Breaths between phrases are notated `[hh]` in the render spec, 180 ms, unvoiced.

## openSMILE gates
- voicing fraction (eGeMAPSv02 `VoicedSegmentsPerSec`): target 0.72, floor 0.60. Album ceiling on track 01 only.
- pitch error RMS ceiling: 4.0 cents (looser than corpus 2.0).
- loudness integrated: -18 LUFS +/- 1.0.
- spectral flux: no hard gate; we expect ordinary speech-like flux here.
