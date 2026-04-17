# 14 To Whom It May Concern — notes

## Why this slot
Closing letter. We list every previous addressee (except the ones we
elide for scansion: the younger self, the diphone engineer, the
distant one, the unreturned letter). The list is the catalog at the
back of the album; the "to you" at the end is the rhetorical handoff.

## Form and motif deployment
Through-composed, 22 bars in C major at 64 bpm. `m.letter_cadence` on
"singletons" — 3400 ms pad. The longest pad on the album and the last
one. After this silence: the album is over.

## reprises
- Every previous track on this album is named (or hinted) in the list.
  This is the only lyric on the album that is explicitly a catalog.
- C major key chosen for plainness, echoing
  `05_to_the_engineer_of_diphones` (also C major) as a tonal rhyme —
  "plain thanks" bookending the back half.

## csdr graph shape
`shaped_noise_bed` at -30 dB, shaped as a very faint combination of
all previous beds' spectral centers averaged. A single Klatt-texture
impulse at bar 22, beat 4.5 — a final "envelope closed" click at
-26 dB. The bed and chain both fade to absolute silence over the last
1600 ms of the 3400 ms pad.

## G2P / pronunciation hints
- "whom" as /h u m/.
- "concern" as /k @ n 's V r n/ — /rn/ cluster at end is handled by
  us1 with the /r/ elided (/k @ n 's V n/) acceptably. We accept the
  elision.
- "lighthouse" as /'l aI t h aU s/ — two syllables; reuse album 02
  treatment.
- "metronome" as /'m E t r @ n oU m/ — reuse album 02 treatment.
- "postmaster" as /'p oU s t m & s t @/ — reuse track 11 treatment.
- "singletons" as /'s I N g @ l t @ n z/ — five phonemic slots; the
  /Ng/ is the soft-n plus stop cluster; us1 handles; final-syllable
  hold on /t @ n z/; let the /z/ decay naturally into the pad.
- Avoid the /Ngl/ onset: insert schwa explicitly between /N g/ and /l/.

## Silero-VAD wiring
Threshold 0.35. The list will produce many short VAD segments (one per
addressee); we expect 10-12 segments in bars 3-14. The final silence
is measured from the fall-edge of the segment containing "singletons"
and must be verified at 3400 ms +/- 30 ms.

## openSMILE gate
Voicing fraction >= 0.55. Pitch-error RMS 2 cents. -16 LUFS integrated
(the long pad will pull the measurement down further than track 13;
we accept).
