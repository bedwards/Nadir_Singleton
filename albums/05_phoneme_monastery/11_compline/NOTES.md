# 11 Compline — notes

## Why this slot
Compline is the final office of the monastic day. After compline the
Great Silence (silentium magnum) begins — no speech until matins the
next morning. We locate it here, three tracks from the album's end,
because the dramatic rupture of `12_silence_rule_broken` must have a
freshly-declared silence to break. Aeolian returns, darker and slower
than any prior minor setting (46 bpm).

## Form and motif deployment
Through-composed, 16 bars at 46 bpm. Structure: /a/ (7 s), /o/ (7 s),
/u/ (7 s) — the brotherhood at rest. A quiet spoken line "the great
silence" appears at bar 7, sustained on A3 with the final `s` drawn
out 2 s — a reprise of `m.letter_cadence` shaped to announce the
silence that follows. The three vowels return in reverse order (u, o,
a) and the track ends on one very long /a/ (10 s) fading by 6 dB over
its final 3 s — the formal handover to silence. `m.vowel_drone`
remains the structural spine.

## csdr graph shape
`fir_cascade` per vowel. `dirac_impulse_bed` at 0.33 Hz — a closing
bell at one click every 3 s. The bell fades by -12 dB across the last
four bars so that the silence after compline is actually silent.

## G2P / pronunciation hints
- `a`/`o`/`u` as established; durations 7 s each in the descent, 10 s
  on the terminal /a/.
- "the great silence" → `D @ g r eI t s aI l @ n s`; recited on A3;
  the final `s` is drawn out as a hissed fricative ≈ 2 s — the
  silence motif announcing itself through its own phoneme.
- "begins" → `b I g I n z`; the `z` is unvoiced to `s` and trails off.
- Breath between vowels is 2 s of measured silence, Silero-VAD
  gated — any stray articulation from MBROLA is edited out.

## openSMILE gates
- voicing fraction: target 0.60, floor 0.50.
- pitch error RMS ceiling: 2.5 cents.
- loudness integrated: -21.0 LUFS.
- silence fraction (custom, Silero-VAD): target minimum 0.20 — the
  night office declares silence by containing it.
