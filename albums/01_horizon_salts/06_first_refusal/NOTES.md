# 06 First Refusal — notes

## Why this slot
Every speaking thing learns to refuse before it learns to demand. We place the refusal here so the album has its first boundary. The refusal is quiet, not angry — a flat "no" that defines a self by what it will not carry.

## Form and motif deployment
22 bars, F# minor (Aeolian), slowed to 66 bpm to sit under the previous track. `m.dawn_utterance` is used once, reversed: a falling minor third (C#4 to A3) then plateau on A3. This inversion registers as a refusal of the original rise. The plateau is held for six full bars on the word "wait".

## csdr graph shape
Same 80–3.2 kHz band-limit. We introduce a notch filter at 250 Hz to carve a space in the bed where the word "no" lives; the notch opens on each "no" and closes between them (driven by VAD onsets from the vocal, in a feedforward chain).

## G2P / pronunciation hints
- "no" as /n oU/ — keep the diphthong full; do not let it shorten.
- "mouth" as /m aU T/ — us3 renders the final /T/ crisply.
- "another" as /V n V D 3`/, three syllables.
- "wait" as /w eI t/ — hold the vowel; the /t/ is a release, not a stop.

## openSMILE gate (primary)
Voicing fraction greater than or equal to 0.7 on the vocal stem. Additional gate: the four "no" instances must show F0 descent of at least 50 cents across their duration (a firm, falling "no" rather than a questioning one).
