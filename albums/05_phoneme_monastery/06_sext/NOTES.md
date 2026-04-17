# 06 Sext — notes

## Why this slot
Sext is the office of the sixth hour, noon — the brightest point of the
monastic day. We braid all three vowels here at their highest tessitura.
G mixolydian brightens via the raised leading tone while keeping the
flattened seventh that mixolydian owes to plainchant history. This is
the peak of the album's light.

## Form and motif deployment
ABCA form, 16 bars at 64 bpm. A: the `a o u` motto held at concert
pitches (A=220, D=294, G=196). B: `la la la / lo lo lo / lu lu lu` —
consonant `l` lets the vowel become a rhythm without losing continuity.
C: nonsense chant `ma na la / mo no lo / mu nu lu` — three consonants
× three vowels, the nine syllables of the hour. Final A returns to
`a o u`. Every held vowel in A is `m.vowel_drone`; the lettered
patterns in B and C are rhythmic textures over the drone bed.

## csdr graph shape
`fir_cascade` switched per vowel (same F1/F2 tables as `04_terce`).
`dirac_impulse_bed` at 2.0 Hz — the noon bell is quick, bright, close.
This is the highest pulse rate on the album.

## G2P / pronunciation hints
- `a o u` as in `04_terce`; held 4 s each in A sections, 1.5 s in
  rhythmic repetitions.
- `la` → `l a`, `lo` → `l @U`, `lu` → `l u:`.
- `ma na la` etc. — consonants are deliberately soft so vowels remain
  primary. Each consonant is 80 ms, each vowel 300 ms on the syllable
  grid.
- Rate of /u/ articulation is 50% of /a/ /o/ to preserve darkness even
  at noon — we still carry the brotherhood's lowest voice.

## openSMILE gates
- voicing fraction: target 0.80, floor 0.70.
- pitch error RMS ceiling: 2.5 cents.
- loudness integrated: -20.0 LUFS.
- spectral centroid Hz: target 900 ± 200 (bright office).
