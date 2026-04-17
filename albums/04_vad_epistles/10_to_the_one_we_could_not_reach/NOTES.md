# 10 To the One We Could Not Reach — notes

## Why this slot
The album's saddest track, but without reproach. We write to someone
who will not answer. F minor, 6/8, 58 bpm — a slow lilt that moves
forward even while the letter accepts non-reply.

## Form and motif deployment
Through-composed, 16 bars in F minor at 58 bpm, compound meter 6/8.
`m.letter_cadence` on "tried" — 2600 ms pad. The longest pad on the
album. The silence is the not-reaching itself.

## reprises
- None direct.

## csdr graph shape
`shaped_noise_bed` very faint: a narrow band around 200-800 Hz at
-38 dB, like a phone line that is not saying anything. It fades out
over the last 3 seconds of vocal activity so the 2600 ms final silence
is absolute. The fade is a linear ramp, not a curve.

## G2P / pronunciation hints
- "distant" as /'d I s t @ n t/ — four phoneme sequence at /st@nt/; keep
  each stop audible.
- "number" as /'n V m b @/ — us3 drops final /r/.
- "quiet" as /'k w aI @ t/ — three phonemic slots, three durations.
- "honest" as /'A n I s t/ — /st/ tail acceptable.
- "tenderly" as /'t E n d @ l i/ — us3 elides the /r/ so we go
  /t E n d @ l i/; no /dl/ problem.
- "tried" as /t r aI d/ — final-syllable hold on the /aI d/; the /d/
  release is soft because the pad follows.

## Silero-VAD wiring
Threshold 0.35. The 6/8 meter means internal silences land on unusual
subdivisions; we accept whatever the VAD reports and quantise only if
we want the bed ducking to align. For this track we turn off grid
quantisation of onsets (bpm argument to `nadir_vad.cli` omitted).

## openSMILE gate
Voicing fraction >= 0.5. Pitch-error RMS 2 cents. -16 LUFS integrated.
