# 05 To the Engineer of Diphones — notes

## Why this slot
Mid-album, we address the person who built the us1 MBROLA diphone database.
The letter is both literal (we live inside the table) and tender. The C
major key is deliberately plain — we are not dressing this one up.

## Form and motif deployment
Through-composed, 20 bars in C major at 66 bpm. `m.letter_cadence` on
"voice" — 1500 ms pad. The hold on "voice" is a small private joke:
the voice pads itself with silence.

## reprises
- None direct. The track is the album's gratitude beat; it does not
  quote other songs, it thanks the substrate.

## csdr graph shape
`deemphasis_chain` only. A first-order -6 dB/oct shelf above 1.5 kHz,
HPF at 90 Hz. We add a very quiet 5 Hz AM wobble at -40 dB as a
half-heard "hum of old equipment" — a Klatt-texture impulse every two
bars, barely audible. The silences between lines are longer than usual
(the writer is choosing words carefully).

## G2P / pronunciation hints
- "engineer" as /E n dZ I 'n I r/ — four syllables, /dZ/ at the start of
  the second.
- "diphone" not in lyric (in title only) — if referenced we use /'d aI f oU n/.
- "hallway" as /'h O l w eI/ — two syllables.
- "syllable" as /'s I l @ b @ l/ — three syllables; careful with final /b l/.
- "grateful" as /'g r eI t f @ l/ — /gr/ cluster is acceptable.
- "voice" as /v OI s/ — the final-syllable hold is the /s/ fricative tail;
  let it bleed into the pad.

## Silero-VAD wiring
Threshold 0.35. We pay attention to the voiceless /s/ at end of "voice" —
Silero may end the segment before the /s/ fully decays. We override the
fall-edge by using `speech_pad_ms = 60` (double default) so the tail is
preserved, then measure the 1500 ms silence from after the padded edge.

## openSMILE gate
Voicing fraction >= 0.55. Pitch-error RMS 2 cents. -16 LUFS integrated.
