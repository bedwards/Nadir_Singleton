# 04 The Name Is Given — notes

## Why this slot
Naming is the third fact after utterance and relation. We had to hear both voices and the room first; now we can risk a name. We keep the name itself unsaid — the lyric only describes it. The Nadir_Singleton never names itself in plain letters.

## Form and motif deployment
24 bars in E minor (Aeolian). `m.dawn_utterance` appears three times: once at bar 5 (E3 rising to G3), once at bar 13 (transposed to B3 rising to D4), and once at bar 21 (collapsed — the plateau stays but the rising third is whispered in the bed, not sung). Each appearance sits under the line "call me".

## csdr graph shape
The band-limit stays 80–3.2 kHz. We add a second FIR stage shaped as a soft formant emphasis around 600 Hz, so the word "call" lands naturally. A `dsb_fc` envelope gates a low sine at 80 Hz on every "call me", giving the word subsonic weight without breaking the dawn palette.

## G2P / pronunciation hints
- "call" as /k O l/ — the /O/ long, the /l/ dark.
- "often" as /O f @ n/, not /O f t @ n/ (us1 handles the silent /t/ poorly; we drop it at G2P time).
- "held" as /h E l d/ — keep the /d/ short.
- "vowel" as /v aU @ l/, two syllables. Do not let G2P collapse to /v aU l/.

## openSMILE gate (primary)
F0 stability on the three "call me" phrases: standard deviation within each phrase under 12 cents. This is the first track where pitch stability is a hard gate; earlier tracks allow more drift.
