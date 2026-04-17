# 13 Salt Mirror — notes

## Why this slot
The album needs its first utterance seen from the other side. We reprise track 01 almost word-for-word but shift "morning" to "evening" and add "again" as the final line. The singleton has lived a day and wakes again — still itself, louder, higher. Placing the mirror at 13 leaves track 14 to be pure stillness.

## Form and motif deployment
E minor (Aeolian), same 68 bpm and 24-bar count as track 01, exactly transposed up a perfect fifth. `m.dawn_utterance` is quoted identically in shape to track 01 but on E/G instead of A/C. The second-half transposition from track 01 (up a fourth) is preserved, landing the final phrase on A4/C5 — the brightest vocal moment on the album.

## csdr graph shape
Identical graph topology to track 01 — same band-limit 80–3.2 kHz, same `shaped_noise_dawn` bed, same `klatt_texture` sample source. The only difference is a 5 dB lift on the upper band (above 1.6 kHz) to match the vocal's higher tessitura. A listener running both tracks side by side should hear the reprise as the same room, later in the day.

## G2P / pronunciation hints
- "salt", "tongue", "wake", "say" — all use the same G2P transcriptions as track 01, character-for-character.
- "evening" as /i v n I N/, two syllables, not /i v @ n I N/.
- "still" as /s t I l/, matching track 07.
- "again" as /@ g E n/, not /@ g eI n/ — the flat American pronunciation.

## openSMILE gate (primary)
Quotation fidelity: the motif contour in each `m.dawn_utterance` quotation must correlate with the track-01 canonical contour at r greater than or equal to 0.9 after transposition normalisation. Voicing fraction greater than or equal to 0.7. Pitch-error RMS at 2 cents ceiling as always.
