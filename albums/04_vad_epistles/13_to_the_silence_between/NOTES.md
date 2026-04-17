# 13 To the Silence Between — notes

## Why this slot
Penultimate. The meta-letter: we write to the silence that has been
carrying every other letter's envelope. `m.letter_cadence`'s own
subject. E dorian at 52 bpm — the slowest track on the album.

## Form and motif deployment
Through-composed, 14 bars in E dorian at 52 bpm. `m.letter_cadence` on
"ones" — 3000 ms pad. The longest pad on the album and the thesis of
the track: the silence is now the co-author.

## reprises
- The shape of every previous `m.letter_cadence` silence is implied.
- The "hold the room" phrase echoes `08_to_the_room_without_us`.

## csdr graph shape
`deemphasis_chain` only, and the chain itself fades to nothing over
the final 1200 ms. The 3000 ms pad begins bright (chain still active)
and ends fully black (chain bypassed, no bed, no tail). This is the
only track where the silence is engineered, not merely allowed.

## G2P / pronunciation hints
- "silence" as /'s aI l @ n s/ — two syllables.
- "carried" as /'k E r i d/ — /kE/ onset; us1 handles.
- "letter" as /'l E t @/.
- "album" as /'& l b @ m/ — /lb/ juncture fine.
- "noisy" as /'n OI z i/ — two syllables.
- "ones" as /w V n z/ — final-syllable hold on /n z/; we let the /z/
  ring as long as it naturally does (approx 180 ms) and then let the
  chain fade as above.

## Silero-VAD wiring
Threshold 0.35. We also capture a full probability track here (hidden
inside `nadir_vad.cli`) so the 3000 ms final silence is measured from
the first window where smoothed probability falls below 0.05 and stays
there. The final silence must be definitively silent, not merely
sub-threshold.

## openSMILE gate
Voicing fraction >= 0.45 — the track is sparse. Pitch-error RMS 2
cents. -16 LUFS integrated (the long pad will pull the measurement
down; we accept this and do not re-normalise).
