# 01 To the Lighthouse — notes

## Why this slot
We open the album by writing back to a character we already know.
Lighthouse Lil (album 02, track 03) kept watch for us; the first letter is
a thank-you. It establishes the epistolary voice without asking the listener
to learn a new character.

## Form and motif deployment
Through-composed, 20 bars in G major at 62 bpm. No refrain. `m.letter_cadence`
appears on the final word "dawn" — we pad 1400 ms of calibrated silence after
the VAD fall-edge. That silence is the envelope being sealed; the next track
picks up from inside it.

## reprises
- Lighthouse Lil (`02_tin_pan_fathom/03_lighthouse_lil`) — the addressee.
  The G major key is borrowed from her track; we lower the bpm from 88 to 62
  to distinguish letter from song.
- The "small room" phrase echoes the rooms referenced across 02.

## csdr graph shape
`shaped_noise_bed` at -28 dB under the vocal, band-limited 120-2400 Hz so it
sits below the vocal formants. No reverb. A slow deemphasis tilt at -3 dB/oct
above 3 kHz to push the voice closer to the ear. The bed ducks 6 dB during
voiced VAD segments and swells back in gaps.

## G2P / pronunciation hints
- "Dear Lil" — two stressed syllables, short gap between. /d I@/ + /l I l/.
- "lamp" as /l & m p/ — do not let the /mp/ cluster close too fast.
- "answers" as /'& n s @ z/ — avoid the /ns/ clash; put a schwa between.
- "window" as /'w I n d oU/ — clean.
- "dawn" as /d O n/ — hold the /O/ long; this is the final-syllable hold
  that the VAD pad sits under. We want the tail to decay naturally before
  the silence begins.

## Silero-VAD wiring
Threshold 0.35. `min_speech_duration_ms = 120`, `min_silence_duration_ms = 80`.
Each line-break in the lyric is a breath point; we confirm the VAD segments
break there, not mid-line. The calibrated 1400 ms tail after "dawn" is
measured from the fall-edge of the last segment.

## openSMILE gate
Voicing fraction >= 0.6 (lower than 01's 0.7; letters breathe more).
Pitch-error RMS 2 cents ceiling. Loudness -16 LUFS integrated.
