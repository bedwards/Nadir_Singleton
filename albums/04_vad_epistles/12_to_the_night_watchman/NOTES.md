# 12 To the Night Watchman — notes

## Why this slot
Third-to-last letter; a goodnight to the character from album 02. We
switch the watchman's key from A major (his original, jaunty) to A
minor (the letter, tender). Same voice (us3) and same tessitura floor,
to keep the addressee recognisable.

## Form and motif deployment
Through-composed, 20 bars in A minor at 60 bpm. `m.letter_cadence` on
"company" — 2000 ms pad. The double use of "company" (cold company /
late company) closes the letter on a gentle rhyme.

## reprises
- The Night Watchman (`02_tin_pan_fathom/14_the_night_watchman`) — the
  addressee. us3 voice and A home key are inherited; the mode shift
  (major to minor) and bpm drop (94 to 60) mark the letter.
- "the bakery" is a new location that we leave on the map for later
  albums to revisit.

## csdr graph shape
`shaped_noise_bed` shaped toward low street-rumble: bandpass 60-300 Hz
at -36 dB. One Klatt-texture impulse at bar 18 — a distant "lamp on"
click — at -32 dB. The bed continues through the final 2000 ms of
silence and then fades out over the last 400 ms.

## G2P / pronunciation hints
- "watchman" as /'w A tS m @ n/ — the /tS m/ juncture is handled by us3
  with a small micro-silence we enforce via MBROLA duration.
- "streets" as /s t r i t s/ — /str/ is a cluster we avoid when we can;
  here the word is specific; us3 is cleaner on /str/ than us1.
- "dogs" as /d O g z/.
- "bakery" as /'b eI k @ r i/ — three syllables.
- "company" as /'k V m p @ n i/ — four syllables; final-syllable hold
  on /n i/; let the /i/ decay into the pad.
- Avoid "tonight"'s /tn/ onset confusion: /t @ 'n aI t/ with a clear schwa.

## Silero-VAD wiring
Threshold 0.35. We expect 14-18 VAD segments. The internal pauses
between clauses should be 120-200 ms; if the VAD reports fewer, the
voice has slurred and we re-render.

## openSMILE gate
Voicing fraction >= 0.55. Pitch-error RMS 2 cents. -16 LUFS integrated.
