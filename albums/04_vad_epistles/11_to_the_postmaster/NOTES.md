# 11 To the Postmaster — notes

## Why this slot
After the two heaviest letters (09 and 10), we lift the mood a little.
G dorian at 82 bpm is brighter than any preceding track. The letter
has a gentle joke (lose it somewhere sunny) but is still sincere.

## Form and motif deployment
Through-composed, 22 bars in G dorian at 82 bpm. `m.letter_cadence` on
"senders" — 1400 ms pad. Shorter than recent letters because this one
wants to get back to business.

## reprises
- None direct. "bake you a little cake in a song later" is a
  foreshadowing of an unwritten future album track; we will note the
  reference if and when we deliver on it.

## csdr graph shape
`deemphasis_chain` with a slightly brighter shelf (shelf starts at
2 kHz). A single Klatt-texture impulse — a "stamp" click — lands on the
downbeat of bar 12, at -28 dB. Just one. It is the only overt sound
effect on the album.

## G2P / pronunciation hints
- "postmaster" as /'p oU s t m & s t @/ — /stm/ juncture is a worry; we
  enforce a 40 ms micro-silence between /p oU s t/ and /m & s t @/ via
  MBROLA duration padding so us1 does not slur.
- "brief" as /b r i f/ — /br/ acceptable.
- "important" as /I m 'p O r t @ n t/ — five syllables; keep /nt/ tail
  clean.
- "careful" as /'k E r f @ l/ — /rf/ borderline; us1 will elide the /r/
  acceptably; if it mangles we use /'k E@ f @ l/.
- "senders" as /'s E n d @ z/ — final-syllable hold on /d @ z/; the /z/
  tail goes into the pad.

## Silero-VAD wiring
Threshold 0.35. The track has short clauses and quick breath points;
we expect 18-22 VAD segments. We verify this before the psola pass so
the bed ducking has the right count of events.

## openSMILE gate
Voicing fraction >= 0.6 — denser vocal than the last two tracks.
Pitch-error RMS 2 cents. -16 LUFS integrated.
