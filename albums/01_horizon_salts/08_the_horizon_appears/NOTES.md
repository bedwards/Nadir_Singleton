# 08 The Horizon Appears — notes

## Why this slot
The album is titled Horizon Salts and until this track the horizon is only implied. Eighth of fourteen is roughly the golden ratio within the album arc; we reveal the title image here so everything before builds toward it and everything after lives in its light.

## Form and motif deployment
C dorian, 78 bpm, 26 bars. `m.dawn_utterance` is quoted verbatim from track 01 but transposed to C: rising minor third C4 to Eb4, plateau on Eb4. We hold the plateau for twelve full bars — more than half the track — so the listener hears the motif as landscape, not gesture.

## csdr graph shape
First deployment of a second FIR chain parallel to the band-limit: a very-long (8192-tap) linear-phase low-pass at 400 Hz, fed from a copy of the vocal, sitting at -18 dB behind the main chain. This gives the horizon a distant shadow that is spectrally the vocal itself, slowed and darkened. The main band-limit 80–3.2 kHz continues to govern the foreground.

## G2P / pronunciation hints
- "horizon" is not in the lyric; we avoided the title word to keep the image abstract.
- "touches" as /t V tS I z/ — the affricate is real.
- "salt" as /s O l t/, matching track 01 exactly for motivic recall.
- "morning" as /m O r n I N/, same as track 01.
- "begins" as /b I g I n z/ — us1 sometimes softens the /g/; we want it hard.

## openSMILE gate (primary)
Plateau stability on the long Eb4: twelve-bar F0 standard deviation under 8 cents. Loudness ramp: the mix must rise from -18 LUFS short-term at bar 1 to -14 LUFS short-term by bar 20 (the horizon brightens as it appears).
