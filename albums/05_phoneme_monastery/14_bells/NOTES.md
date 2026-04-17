# 14 Bells — notes

## Why this slot
The penultimate track. After return, the tower bells ring across the
cloister — not as calls to office (we have no more offices in the day)
but as the stone's own voice repeating what it has heard. The bells
are the `dirac_impulse_bed` made foreground: elsewhere we used it as
subtle pulse; here it is the primary event, with the vowels as
resonance trapped in the arches. This is the album's load-bearing
reprise of `m.room_impulse` from albums 06 and 14.

## Form and motif deployment
Through-composed, 14 bars at 60 bpm. Each vowel is a single bell-strike
plus the held resonance: we attack with a dirac click at time 0 and
the MBROLA vowel enters 300 ms later at full level, held 5–7 s. The
vowel IS the bell's afterglow. Eight strikes over 14 bars, vowels
cycling `a, a, o, a, u, a, a o u, a, o, u, a, a`. `m.vowel_drone`
carried by every held vowel; `m.room_impulse` carried by the
foregrounded Dirac bed and its role in sculpting each vowel's attack.

## csdr graph shape
`dirac_impulse_bed` at 0.25 Hz as the primary sample source — the
clicks are not a bed here but the lead texture (CORPUS.md: "csdr
Dirac-click bed, vary IR length"). We vary the `peaks_fir_cc` tap
count per strike: 512 taps on smaller strikes (closer bells), 2048
taps on the big strikes at bars 1, 8, 14 (large tower bell). The
vowel passes through a `fir_cascade` tuned to the current vowel's
formants and is mixed at unity with the bell strike's resonant tail —
so that the ear cannot tell where the bell ends and the vowel begins.

## G2P / pronunciation hints
- `a` → SAMPA `a`; 5–7 s held, attack delay 300 ms behind the dirac
  click, amplitude envelope rises 0→full over 500 ms.
- `o` → `O`; same attack profile.
- `u` → `u:`; same attack profile; slightly longer tail (8 s) on
  bar 11's strike — the darkest bell.
- `a o u` at bar 8 is the triad as a single chord strike: all three
  vowels attack together at bar 8 beat 1, each held 6 s. This is the
  largest bell of the piece.

## openSMILE gates
- voicing fraction: target 0.55, floor 0.45 (strikes and silences
  alternate).
- pitch error RMS ceiling: 3.5 cents (the bell's pitch anchors the
  vowel's pitch, so the cascade must phase-align).
- loudness integrated: -20.0 LUFS.
- IR decay slope: per strike, -60 dB over 6 s — monastery stone
  signature.
