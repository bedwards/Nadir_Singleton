# 08 To the Room Without Us — notes

## Why this slot
After the direct letter to the listener, we turn to absence. This is
the quietest track on the album. B minor at 54 bpm — slow and low.
us3 voice for tonal contrast with us1.

## Form and motif deployment
Through-composed, 16 bars in B minor at 54 bpm. `m.letter_cadence` on
"missing" — 2400 ms pad. The silence here is structurally doing the
heavy lifting; the lyric is sparse on purpose.

## reprises
- None direct. The track's spatial subject invites comparison with
  album 02 `16_old_piano_in_the_hall` but we do not quote it.

## csdr graph shape
`shaped_noise_bed` shaped toward a faint room-tone: pink-ish noise
low-passed at 1.2 kHz, sitting at -36 dB. The bed continues through
the entire 2400 ms final silence — the room tone persists when we stop
speaking, which is the whole thesis.

## G2P / pronunciation hints
- "empty" as /'E m p t i/ — /mpt/ cluster is borderline; us3 handles
  it; if the psola pass shows irregularity we reduce to /'E m t i/.
- "light" as /l aI t/.
- "floor" as /f l O/ — us3 drops final /r/; no /fl/ problem.
- "clock" as /k l A k/ — /kl/ cluster acceptable.
- "chair" as /tS E/ — us3 drops the /r/.
- "missing" as /'m I s I N/ — final-syllable hold is on /I N/; let the
  nasal release into the pad.

## Silero-VAD wiring
Threshold 0.35. `min_silence_duration_ms = 200` — long line gaps are
part of the poem. The bed makes the silence "something," not "nothing";
VAD should still classify it as non-speech because the bed energy sits
below the speech band.

## openSMILE gate
Voicing fraction >= 0.5 — sparse vocal, plenty of silence. Pitch-error
RMS 2 cents. -16 LUFS integrated (the integrated measure includes the
long tail so the vocal itself runs slightly hotter than -16 LUFS momentary).
