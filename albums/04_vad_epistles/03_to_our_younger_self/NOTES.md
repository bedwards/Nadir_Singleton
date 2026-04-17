# 03 To Our Younger Self — notes

## Why this slot
After two letters outward, we turn inward. This letter is addressed to the
singleton at its first utterance (album 01). A minor mode and 3/4 meter
change the weather: thoughtful, almost lullaby-shaped, not sad.

## Form and motif deployment
Through-composed, 18 bars in A minor at 56 bpm, 3/4. `m.letter_cadence` on
"ones" — the longest pad yet, 2000 ms. We linger because the addressee
is our own past and the distance is the whole album.

## reprises
- First Salt (`01_horizon_salts/01_first_salt`) — the addressee. We borrow
  A minor directly from that track. "salt" and "morning" and "first word"
  are all quoted from it.
- Memory Hymn (`01_horizon_salts/12_memory_hymn`) — tonal neighbour.

## csdr graph shape
`deemphasis_chain` (preset): a first-order -6 dB/oct shelf above 1.5 kHz,
then a linear-phase FIR gentle HPF at 90 Hz. No shaped-noise bed on this
track; the deemphasis carries the whole "close to the ear" color. One
stem only: vocal + chain. The silence after the letter is absolute.

## G2P / pronunciation hints
- "small one" — two clean syllables; do not elide. /s m O l/ + /w V n/.
- "salt" as /s O l t/ — identical to album 01; dark /l/.
- "mistake" as /m I 's t eI k/ — clean /st/ not /mst/.
- "patient" as /'p eI S @ n t/ — palatalised /S/.
- "ones" as /w V n z/ — the final-syllable hold is the /n z/ decay;
  let the /z/ taper before the silence begins.
- Avoid the /rl/ cluster: "yours" as /j O z/.

## Silero-VAD wiring
Threshold 0.35. We run a second pass at threshold 0.25 to confirm the
very soft "be slow / be patient" lines register as speech and not as
inter-letter silence. 2000 ms tail after "ones".

## openSMILE gate
Voicing fraction >= 0.5 — the softest letter yet; we expect unvoiced
regions. Pitch-error RMS 2 cents. -16 LUFS integrated.
