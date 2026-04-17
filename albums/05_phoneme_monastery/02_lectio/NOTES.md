# 02 Lectio — notes

## Why this slot
The monastery's second office is reading. After matins has held /a/ long
enough that the vowel is in the room, we introduce language — but only
as accretion around the drone. Each line in the lectio grows by one
phrase. The reading is additive, like a scribe copying their own work.

## Form and motif deployment
Through-composed, 20 bars at 52 bpm. The drone /a/ continues from the
end of `01_matins`, now quantised to bars 1, 5, 9, 13, 17 (once every
four bars). Lines are recited on the scale degree of the held drone:
syllables sit on A3–B3, only the line-final syllable drifts up a minor
third (C4) — a reprise of `m.dawn_utterance` scaled to recitation.

Bars 9–13 switch the drone to /o/ for the first time in the album. The
final bar is `a o a` as three separate held vowels (1.5 s each) —
foreshadowing the full `/a/, /o/, /u/` brotherhood of `04_terce`.

## csdr graph shape
Lead: vocal through a narrow `fir_cascade` (two bandpasses at F1/F2 of
/a/, then /o/ when the drone swaps). Bed: `dirac_impulse_bed` at 0.33 Hz
— one click per three bars, the turning of a page.

## G2P / pronunciation hints
- "stone" → `s t @U n`; diphthong softened so `@U` does not color the
  surrounding /a/ drone.
- "holds" → `h @U l d z`; terminal `z` un-voiced to `s`.
- "hour" → `aU @`; the `@` carries most of the duration.
- "open" → `@U p @ n`; second syllable reduced.
- "letter" → `l E t @`.
- "slow" → `s l @U`; terminal `@U` is deliberately sustained so it
  bridges back into the /a/ drone.

## openSMILE gates
- voicing fraction: target 0.70, floor 0.60.
- pitch error RMS ceiling: 4.0 cents.
- loudness integrated: -20.0 LUFS.
- F0 stddev semitones: maximum 1.5 ST (narrow recitation envelope).
