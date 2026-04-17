# 09 To the Unreturned Letter — notes

## Why this slot
A letter addressed to a letter. The recursion is deliberate: we are
writing the album about writing, and this is the most reflexive track.
D minor, 64 bpm. Tender, not anxious.

## Form and motif deployment
Through-composed, 20 bars in D minor at 64 bpm. `m.letter_cadence` on
"envelope" — 2000 ms pad. The signature ("the envelope") is a small
punchline that resolves the recursion; the pad lets it breathe.

## reprises
- None direct. Thematic kinship with `02_from_the_broken_metronome`
  (who also signs off with a non-name) but no quotation.

## csdr graph shape
`deemphasis_chain` only. Single stem. A single Klatt-texture impulse
at bar 1 (a tiny "click" like a mail slot) to mark the letter going
out; the impulse is at -30 dB and lasts under 20 ms.

## G2P / pronunciation hints
- "tuesday" as /'t u z d eI/ — two syllables.
- "weather" as /'w E D @/.
- "opens" as /'oU p @ n z/ — three-syllable feel; clean /p @ n/.
- "resting" as /'r E s t I N/ — /st/ acceptable.
- "envelope" as /'E n v @ l oU p/ — four syllables; the /nv/ cluster is
  fine. Final-syllable hold on /l oU p/; the /p/ release is the sharp
  edge before the pad. Let the silence swallow the /p/'s transient.
- Avoid "letter's" possessive clusters — reword.

## Silero-VAD wiring
Threshold 0.35. We note that the final /p/ of "envelope" is an
unvoiced stop; Silero may end the segment on the preceding /oU/ and
drop the /p/. We protect the /p/ with `speech_pad_ms = 80`.

## openSMILE gate
Voicing fraction >= 0.55. Pitch-error RMS 2 cents. -16 LUFS integrated.
