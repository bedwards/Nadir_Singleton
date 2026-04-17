# 07 To the Listener — notes

## Why this slot
The album's pivot. Seven letters in, we stop writing to characters and
write to the person listening. F major at 70 bpm is the plainest weather
we have; the song is deliberately unadorned so the second-person address
lands.

## Form and motif deployment
Through-composed, 20 bars in F major at 70 bpm. `m.letter_cadence` on
"voices" — 2200 ms pad, the longest hold on the album so far. The long
silence is intended to make the listener aware that the letter was to
them specifically.

## reprises
- None direct. This track is the album speaking out of character.

## csdr graph shape
`deemphasis_chain` only. No shaped noise, no Klatt impulses. The vocal
is the whole song. We run the chain with a slightly brighter shelf
(-4.5 dB/oct above 1.8 kHz rather than 1.5 kHz) so the voice feels
closer and clearer than the other letters.

## G2P / pronunciation hints
- "hello" as /h E 'l oU/ — two syllables.
- "listener" as /'l I s @ n @/ — three syllables, us1 drops final /r/.
- "weather" as /'w E D @/ — voiced /D/.
- "particular" as /p @ 'r t I k j @ l @/ — five syllables; the word is
  difficult; us1 handles it but we schedule a small gap before and after
  so the listener can follow.
- "voices" as /'v OI s I z/ — two syllables; final-syllable hold is on
  /s I z/; let the /z/ decay into the pad.
- No /rl/, no /str/ at onset, no /kn/.

## Silero-VAD wiring
Threshold 0.35. `min_silence_duration_ms = 150` — we expect deliberate
pauses between lines. The 2200 ms final tail is the centerpiece of
`m.letter_cadence` on this album; we want this to be the silence that
makes a listener notice.

## openSMILE gate
Voicing fraction >= 0.6 — this track is the least breathy. Pitch-error
RMS 2 cents. -16 LUFS integrated.
