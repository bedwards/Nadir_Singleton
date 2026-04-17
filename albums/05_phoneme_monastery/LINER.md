# Phoneme Monastery

_Album 5 of 20 in the Nadir_Singleton corpus._

ritual vowel drones; a brotherhood of /a/, /o/, /u/.

## Motif palette
vowel drones, chant

## Arc (15 tracks)

The album follows one liturgical day inside a monastery whose rule is
phonemic: the community prays in held vowels, the offices are marked by
which vowel is permitted, and a single broken silence near the end of
the day is recovered by return.

| # | slug | office / scene |
|---|------|----------------|
| 01 | matins | matins — the first /a/ at 3 a.m. |
| 02 | lectio | lectio — spoken reading over the drone |
| 03 | lauds | lauds — dawn praise, /o/ enters |
| 04 | terce | terce — third hour, /u/ enters |
| 05 | chapter_of_faults | confession, near-whispered |
| 06 | sext | noon office, three vowels braided |
| 07 | none | ninth hour, the drone thins |
| 08 | vigil | long watch of `/a/` under darkness |
| 09 | vespers | evening office, candle-lit |
| 10 | collatio | gathering for reading — two voices |
| 11 | compline | night office, descent toward silence |
| 12 | silence_rule_broken | a single transgressive consonant |
| 13 | return | reintegration, /a/ recovered |
| 14 | bells | tower bells — dirac impulse bed |
| 15 | amen | closing on a held /a/ → silence |

## Motif deployment
- `m.vowel_drone` is the through-motif: every track except `silence_rule_broken`
  carries at least one held 6–9 s vowel.
- Reprises: `m.dawn_utterance` in `matins`/`lauds`, `m.letter_cadence` in
  `compline`, `m.feature_vector_waltz` in `collatio`, `m.tin_pan_turnaround`
  ghost-quoted in `vespers`, `m.room_impulse` in `bells`.
- `silence_rule_broken` deliberately refuses `m.vowel_drone` — its absence
  is the dramatic event.

## Production notes
- Slow tempi throughout (bpm 40–72), modal scales (minor, dorian, phrygian,
  aeolian, mixolydian), long bars.
- Narrow tessitura around [140, 330] Hz — voices sit low-mid, a choir of
  men at rest.
- `vox_loudness_lufs = -20` across the album: this is quiet devotion, not
  performance.
- Beds are `dirac_impulse_bed` (slow pulse, bell-like) or `fir_cascade`
  (bandpass chain, psalm-shaped). No shaped noise unless cloistered scene
  demands a wash.
- Lyrics are sparse. Many tracks are one syllable repeated with variation
  or nonsense chant syllables in SAMPA-valid phonemes for `us1`/`us3`
  (`a`, `O`, `u:`, `@`, `i:`, plus consonants `m`, `n`, `l`, `h`, `s`).
- Voice alternates `us1` (brighter, younger cantor) and `us3` (darker,
  elder) by office — `us3` at night, `us1` at dawn, both braided at `collatio`.
