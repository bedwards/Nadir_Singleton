# Pitch Percentile Parade — notes

Track 01 opened with F0 stats as mean/median. Track 15 returns to F0 stats but
focuses on the percentile family: `F0semitoneFrom27.5Hz_sma3nz` functionals at
20th, 50th, and 80th percentiles. A parade of floats literalises the ordered
procession of quantiles.

## Slot rationale
Placing a second F0-family track just before the closer gives the album a
structural bookend. The opener used mean/median; this penultimate track uses
percentiles. Together they form a compositional rhyme: stats open, stats close.

## Motif deployment
- 3/4, 138 bpm (matches track 02 tempo) to suggest we are parading back past
  the barker's corner.
- Beat 1 = 20th percentile F0, beat 2 = 50th percentile, beat 3 = 80th.
- A sections: 20th climbs while 80th holds.
- A' sections: 80th descends while 20th holds.
- B: all three collapse onto the median for 4 bars (the parade halts).

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> praat psola retarget (F0 scheduled to hit percentiles per bar)
  |-> ring_mod_multi (carriers on 20th/50th/80th) -> bed
  |-> granular_texture (grain triggered on each beat by percentile) -> texture
  '-> mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "percentile" as `p @ r 's E n t aI l` — light first syllable.
- "quantile" as `'k w A n t aI l`, "parade" as `p @ 'r eI d`.
- "evening" as `'i v n I N` — short medial schwa is fine.

## openSMILE gates
- F0 20th percentile within plan +/- 1.5 semitones.
- F0 80th percentile within plan +/- 1.5 semitones.
- F0 median (50th) within +/- 1 semitone.
- F0 RMS error < 2 cents on voiced frames.
- Voicing fraction > 0.7.
