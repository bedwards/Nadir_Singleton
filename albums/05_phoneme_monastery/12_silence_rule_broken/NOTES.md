# 12 Silence Rule Broken — notes

## Why this slot
The dramatic rupture of the album. The Great Silence declared at the
end of compline is broken — not by a word, not by a vowel, but by a
consonant slipping out in the dark. We deliberately refuse to carry
`m.vowel_drone` here: this is the only track on the album without the
through-motif. Its absence IS the dramatic event. The tempo is
noticeably faster (72 bpm, near the top of the album's 40–72 window)
to register the adrenaline of transgression.

## Form and motif deployment
Through-composed, 10 bars at 72 bpm — the shortest track on the album.
Voiceless plosives `t`, `k`, `p` appear one at a time, each followed
by a bar of silence. Then a triple volley `t k p / t k p / t k p` —
this is the reprise of `m.plosive_rhythm` (album 17), quoted early
into the corpus from a future album. A single sung confession "one
of us spoke in the dark" is the track's only vowel-bearing line.
`m.letter_cadence` closes — a final isolated `t` then the word
"silence" whispered, unsustained.

## csdr graph shape
`dirac_impulse_bed` at 4.0 Hz — the fastest pulse on the album, a
restless clock at the moment of rupture. The bed runs alongside the
voice but is no longer separable from it: each consonant `t`/`k`/`p`
is a real plosive, tracked by Silero-VAD, and its onset retriggers a
`dirac_impulse_bed` click one-to-one. The bed and the transgression
are identical. A very narrow `fir_cascade` passes only the 2–6 kHz
band — emphasising sibilant and plosive energy while excluding the
vowel drone frequencies. The drones literally cannot pass through
this filter.

## G2P / pronunciation hints
- `t` → SAMPA `t`; each plosive is 80 ms total, 40 ms of occlusion
  followed by 40 ms of aspirated release.
- `k` → `k`; 90 ms, slightly longer aspirated release.
- `p` → `p`; 70 ms, bilabial release.
- "one of us spoke" → `w V n @ v V s s p @U k`; sung (tentatively, not
  confidently) on A3; the `@U` is clipped to 300 ms — the speaker is
  already retreating into silence.
- "in the dark" → `I n D @ d A: k`; terminal `k` is the last audible
  phoneme before the final "silence" word trails off unvoiced.
- "silence" → whispered only, `s aI l @ n s`; all unvoiced, trailing.

## openSMILE gates
- voicing fraction: target 0.20, floor 0.10 (mostly unvoiced — this
  is a track of consonants).
- pitch error RMS ceiling: 4.0 cents (wider — little sustained pitch
  to track).
- loudness integrated: -22.0 LUFS (the transgression is quiet).
- plosive onset density (Silero-VAD derived): minimum 8 onsets over
  the 10-bar span — quality gate on the rhythm motif.
