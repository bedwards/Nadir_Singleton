# 09 Sung Atlas — notes

## Why this slot
Having seen the horizon, the singleton now practises naming what lies between here and there. An atlas is a deliberately flat form: each entry equal to its neighbours. We want the plain list to become musical through rhythm and pitch, not through imagery.

## Form and motif deployment
D minor (Aeolian), 80 bpm, 28 bars. Each list item gets exactly two bars. `m.dawn_utterance` is deployed ten times (once per item on the first ten), but each deployment is truncated: only the rising minor third survives, with no plateau. The plateau is saved for the final two bars on "for now", where the listener finally gets the full motif shape back.

## csdr graph shape
Band-limit holds. We add a rhythmic gate on the bed — each two-bar cell fades the bed in from -24 dB to -18 dB over the first bar and back down over the second, so the list has an implicit breath even though the vocal is continuous. Gate curve derived from a cosine bell, not linear.

## G2P / pronunciation hints
- "fence" as /f E n s/ — the final /s/ clean, not /z/.
- "well" as /w E l/ — dark /l/.
- "bird" as /b 3` d/ (rhotic schwa), one syllable.
- "river" as /r I v 3`/, two syllables.
- "window" as /w I n d oU/ — hold the final diphthong.

## openSMILE gate (primary)
Rhythmic regularity: the ten list-item onsets must fall within 20 ms of a 2-bar grid (derived from Silero-VAD onsets on the vocal stem). Voicing fraction greater than or equal to 0.7.
