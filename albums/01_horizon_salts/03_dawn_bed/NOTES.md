# 03 Dawn Bed — notes

## Why this slot
Two voices have spoken; now we pause. Track 03 is almost textural — we name the room before the room fills. Placing an interlude early teaches the listener that silence and bed are members of this music, not absence.

## Form and motif deployment
32 bars, slow 60 bpm. The MBROLA vocal is sparse — four short lines spread across 32 bars, roughly one line every 8 bars. `m.dawn_utterance` appears only once, at bar 25, in the vocal: a rising minor third from D3 to F3 on the word "light". The rest of the time the motif lives in the bed as a filter sweep, not a pitch gesture.

## csdr graph shape
Wider bed than tracks 01–02. Two parallel branches of `shaped_noise_dawn`: one high (1.8–3.2 kHz) and one low (80–240 Hz), each band-limited, summed at -6 dB. A slow FIR sweep (center frequency drifting from 240 Hz up to 1.8 kHz across 32 bars) bridges them. The vocal sits in the remaining gap.

## G2P / pronunciation hints
- "sill" as /s I l/ — short and bright.
- "floor" as /f l O r/ (single syllable, rhotic).
- "only" as /oU n l i/, not /A n l i/ — the first vowel is round.
- Hold the final "light" long: we want the /aI/ diphthong to decay slowly into the bed.

## openSMILE gate (primary)
Because this is bed-dominant, the primary gate is spectral-flux density on the bed: 0.3 to 0.6 (we do not want it dead, nor busy). Voicing fraction on the four vocal lines still needs to clear 0.7; the silence around them is not measured.
