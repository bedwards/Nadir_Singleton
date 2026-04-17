# 07 None — notes

## Why this slot
None is the ninth hour, mid-afternoon. Historically the office where
energy ebbs — monks nap between none and vespers. We honour that
with a deliberate thinning: fewer repetitions, longer silences between
drones, a single spoken couplet emerging from the held vowels like a
remembered phrase. Aeolian again — home.

## Form and motif deployment
Through-composed, 14 bars at 56 bpm. /a/ appears three times in
descending register (A3, G3, F3), then /o/ twice, then /u/ once, then
a small return of /a/, /o/, /a/ at their original registers.
`m.vowel_drone` dominates, but each drone is shorter than in earlier
offices (5 s, not 8 s) — the thinning. A single sung couplet ("the
light slants low …") emerges as if half-dreamt over the drone, on a
level A3. Final bar: /a/ held 9 s, the longest of the track — we
gather ourselves before vespers.

## csdr graph shape
`fir_cascade` tuned per vowel. `dirac_impulse_bed` at 0.75 Hz — the
afternoon bell has slowed from noon's 2.0 Hz but not to matins'
contemplative 0.5. We sit between wake and rest.

## G2P / pronunciation hints
- `a`/`o`/`u` vowels as established; hold duration 5 s in the descent
  sequence, 9 s in the final.
- "light" → `l aI t`; diphthong `aI` briefly approaches the /a/ drone
  before the terminal `t`.
- "slants" → `s l a n t s`; the `a` in the middle is the open vowel —
  we deliberately let it rhyme with the drone.
- "low" → `l @U`; sustained to 1.2 s so it bridges into the next /o/.
- "across" → `@ k r Q s`; `Q` is a dark vowel appropriate for the
  afternoon shadow.
- "stone" → `s t @U n`; unreleased `n` lets silence follow naturally.

## openSMILE gates
- voicing fraction: target 0.65, floor 0.55.
- pitch error RMS ceiling: 3.0 cents.
- loudness integrated: -20.5 LUFS (slightly quieter — the ebb).
- F0 stddev semitones: maximum 0.4 ST across any held vowel.
