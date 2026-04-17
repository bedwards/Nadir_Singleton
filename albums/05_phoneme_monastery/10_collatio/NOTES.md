# 10 Collatio — notes

## Why this slot
Collatio is the community meeting after vespers: a shared reading,
often Cassian's Conferences, followed by conversation before the Great
Silence begins at compline. We place it here as the album's only duet.
Two MBROLA voices (`us1` as primary upper, `us3` as secondary lower
routed under) perform the track. The manifest's `mbrola_voice` records
the lead; NOTES details the second voice. 3/4 meter continues vespers'
triple pulse — the gathering holds the evening's tempo.

## Form and motif deployment
18 bars at 56 bpm, through the duet throughout. `us1` holds /a/ while
`us3` holds /o/ a fourth below; they swap on each pair. The overlap is
the social moment of the office — the community is two voices sharing
a drone at different registers. `m.feature_vector_waltz` (album 03)
reprises as the triple-meter structure where each beat of the 3/4 bar
is shaped by one eGeMAPS family (beat 1: loudness; beat 2: spectral;
beat 3: voicing). openSMILE analysis per beat retargets the next bar's
microdynamics — the waltz is now a listening practice, not a dance.

## csdr graph shape
Two parallel `fir_cascade` chains, one tuned to /a/ for the lead vocal,
one tuned to /o/ for the secondary; their outputs summed via csdr
mixer with per-voice pan (lead center, secondary -0.3 left).
`dirac_impulse_bed` at 0.75 Hz — a gentle triple-adjacent pulse.

## G2P / pronunciation hints
- Lead `us1`: `a`/`O`/`u:` at tessitura 220–310 Hz.
- Secondary `us3`: `a`/`O`/`u:` at tessitura 150–200 Hz — a perfect
  fourth below the lead.
- Phrases are swapped: when lead holds /a/, secondary holds /o/; when
  lead holds /o/, secondary holds /a/. `u:` enters together in the
  `a o u / u o a` braid (bars 13–14).
- "we gather slow" → lead only: `w i: g a D @ s l @U`; spoken on D4
  with terminal `s l @U` sustained 1 s.
- "we sing" → `w i: s I N`; terminal `N` is unreleased and long.

## openSMILE gates
- voicing fraction (lead+sec summed): target 0.75, floor 0.65.
- pitch error RMS ceiling (per voice): 3.0 cents.
- loudness integrated: -20.0 LUFS.
- harmonic-to-noise ratio (HNR): target 18 dB — the two drones must
  lock spectrally.
