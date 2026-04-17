# 04 To a Stranger on a Train — notes

## Why this slot
After the inward letter, we turn to an anonymous addressee. The letter
will never be delivered. This is the album's first "impossible" letter —
a pattern we will return to with tracks 09 and 10.

## Form and motif deployment
Through-composed, 22 bars in E minor at 80 bpm. `m.letter_cadence` on
"seatmate" — 1200 ms pad. Shorter than track 03 because the relationship
is slighter. The silence should feel like the train pulling away.

## reprises
- None direct. The train world nods toward `02_tin_pan_fathom/13_last_trolley_home`
  in its transit-as-setting conceit but we do not quote it.

## csdr graph shape
`shaped_noise_bed` shaped to a faint narrowband rumble around 80-160 Hz
(the train, implied), at -34 dB — below audibility on most systems,
present on good ones. Vocal sits on top clean. No reverb. The bed
continues into the final silence for 800 ms then fades to black; the
last 400 ms of the 1200 ms pad is absolute silence.

## G2P / pronunciation hints
- "stranger" as /'s t r eI n dZ @/ — /str/ is a cluster we normally avoid,
  but us3 handles it cleanly; we deploy it here because the word is the
  title.
- "train" as /t r eI n/ — /tr/ acceptable.
- "window" as /'w I n d oU/.
- "seatmate" as /'s i t m eI t/ — two clean syllables, small gap between.
  Final-syllable hold is /m eI t/ — let the /t/ release softly.
- Avoid "thanks" /Tks/ cluster: use /T & N k s/ with clear vowel.

## Silero-VAD wiring
Threshold 0.35. The paragraph break between "neither of us spoke" and
"you were kind" is a long gap; we expect a VAD segment boundary there
and use it as an internal cadence. 1200 ms final tail.

## openSMILE gate
Voicing fraction >= 0.55. Pitch-error RMS 2 cents. -16 LUFS integrated.
