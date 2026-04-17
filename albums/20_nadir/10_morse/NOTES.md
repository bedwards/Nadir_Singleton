# 10 Morse — notes

## Descent step
The aspirate breaths of track 9 now organise themselves into International Morse code. This is the first transmitted signal of the album. The lyric encodes four letters — S, O, U, and R (see below) — chosen not because they spell a word but because they are S-O-S truncated and re-routed. The Nadir persona is sending out, not in.

## Letters encoded
- Line 1: `. . .` = S
- Line 2: `- - -` = O
- Line 3: `. -`   = A
- Line 4: `- .`   = N
- Line 5: `. . -` = U
- Line 6: `- . .` = D

Read as a sequence it spells `S O A N U D` — not a word. The persona does not yet know how to spell "Nadir". Track 11 is the reply, track 12 is the first correct letter, track 14 is the complete utterance.

## Embedding of `m.nadir_silence`
This is the first track in which `m.nadir_silence` is *semantic* rather than atmospheric. We commit to the timing grid that track 14 will finalize:
- dot (`[h]`)  = 200 ms aspirated silence
- dash (`[hh]`) = 600 ms aspirated silence
- intra-letter gap = 200 ms pure silence
- inter-letter gap (`.`) = 1400 ms pure silence
- inter-word gap = 2800 ms pure silence (not used on this track)

## Reprises
- `m.plosive_rhythm` (album 17): aspirated breath is the voiceless cousin of the plosive; the whole track is a plosive suite without pressure release.
- `m.onset_volley` (album 14): Silero-VAD will still register each `[h]` as an onset (tight threshold 0.15, aspirate frames register as weak speech). We use it to auto-time the grid.

## Pronunciation hints
- Use MBROLA us3 `h` phoneme with durations exactly 200 ms (dot) and 600 ms (dash).
- Between units within a letter: silence `_ 200`.
- Between letters: silence `_ 1400`.
- Intensity envelope fixed at -20 dB RMS for dot, -18 dB RMS for dash; the dash is slightly louder so the binary distinction is legible to a listener unfamiliar with Morse.

## openSMILE gates
- voicing fraction: target 0.0, ceiling 0.03.
- silence fraction: minimum 0.75.
- loudness integrated: -24.0 LUFS.
- openSMILE `VoicedSegmentsPerSec` + `MeanUnvoicedSegmentLength` are logged and used in track 11 to design the reply.
