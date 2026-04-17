# 08 Vigil — notes

## Why this slot
The vigil is the long night watch; historically the longest office of
the rule. We place it mid-album, between none and vespers, to sit at
the album's structural bottom. This is the slowest track (40 bpm) and
the one that most strictly refuses language. Eight /a/'s in 24 bars,
each held 9 s — the canonical `m.vowel_drone` stretched to its album
maximum. E phrygian lends a steady, ancient darkness without any major
coloration.

## Form and motif deployment
Through-composed. 24 bars at 40 bpm = 144 s of near-continuous /a/
on E3 (165 Hz). Eight held vowels spaced with 3 s of silence between.
No words. No second vowel. No consonants. The piece is a discipline —
to watch is to hold one thing and not look away. `m.vowel_drone` is
the entire structure.

Micro-variation: each successive /a/ adds one increment of vibrato
depth, from 0.15 ST (first) to 0.5 ST (eighth). The ear cannot name
the difference but feels the drone tiring across the watch.

## csdr graph shape
`dirac_impulse_bed` at 0.25 Hz — one click every 4 s, a lantern tick
in stone. `fir_cascade` tuned to /a/ F1/F2 (720/1240 Hz) only —
narrower than other tracks, no F3. The vowel is pared to its minimum
legible shape.

## G2P / pronunciation hints
- `a` → SAMPA `a`; 9 s hold per occurrence.
- Each vowel starts at voicing fraction 0.9, glides to 0.7 at its
  middle, back to 0.9 at end. Praat PSOLA animates this as a breath
  envelope — the singer tiring then recovering within a single vowel.
- Silence between vowels is precisely 3.0 s (Silero-VAD gated). No
  breath sounds leak into the silence.

## openSMILE gates
- voicing fraction: target 0.55, floor 0.45 (half the piece is
  silence).
- pitch error RMS ceiling: 2.5 cents (the drone must be true).
- loudness integrated: -21.0 LUFS.
- F0 stddev semitones across any single held vowel: maximum 0.5 ST.
