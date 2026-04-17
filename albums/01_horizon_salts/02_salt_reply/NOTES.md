# 02 Salt Reply — notes

## Why this slot
The first utterance in track 01 is the spark; every spark requires a second thing to call it fire. We place the reply here so the album establishes relation as its second fact. The voice is us3 (darker, slightly lower), answering us1's first salt.

## Form and motif deployment
Through-composed, 28 bars. We echo `m.dawn_utterance` from track 01 but invert the rising third: A3 is answered by C4 descending to A3, then plateau on A3. The effect is a bow of acknowledgement. Bars 17–28 lift the motif a whole step, and for the first time both voices overlap for four bars before separating.

## csdr graph shape
Same band-limit 80–3.2 kHz as track 01, but we pan the reply -0.4 (left-of-centre) while the bed stays centred. A `shift_addition_cc` at +3 Hz on the texture bed produces a very slow beating against the us1 residue, sonically linking tracks 01 and 02.

## G2P / pronunciation hints
- "heard" as /h 3` d/ (rhotic schwa), not /h E r d/.
- "warmer" as /w O r m 3`/, two syllables.
- "lower" as /l oU 3`/ — we want the diphthong clear, not collapsed.
- "tongue" again /t V N/ as in track 01, for consistency.

## openSMILE gate (primary)
Voicing fraction greater than or equal to 0.7 on both vocal entries. Additional gate: spectral flux between the two voices must stay within 0.15 of each other (we do not want one voice brighter than the other).
