# 05 Chapter of Faults — notes

## Why this slot
The chapter of faults is the monastic rite of public confession. We
place it mid-morning, when the community has assembled and energy is
clearest. Each fault in the lyric is a phonemic fault: holding a vowel
wrong, letting a consonant slip, attending only to one's own voice.
Aeolian (natural minor) returns us to the album's home modality —
contrition sung over the same drone that opened matins.

## Form and motif deployment
Verse/response over 20 bars at 48 bpm. Each cycle is: held /a/ drone
(4 s) followed by a single confessional line spoken on monotone at
A3. `m.vowel_drone` underscores every verse. The spoken responses are
half-whispered (voicing fraction 0.4) — this is the quietest track so
far (LUFS target -22). `m.letter_cadence` (from album 04) is reprised:
every confessional line ends with a silence-padded final syllable
before the next drone enters.

## csdr graph shape
`fir_cascade` tuned to /a/ F1/F2 (720/1240 Hz) for the drones.
`dirac_impulse_bed` at 0.25 Hz — a slow gavel, the abbot marking each
confession. The pulse is deliberately slower than matins; confession
lengthens the hour.

## G2P / pronunciation hints
- `a` → SAMPA `a`; each held 4 s (shorter than other offices — faults
  are brief).
- "vowel" → `v aU @ l`; the `aU` softened to avoid a diphthong clash.
- "consonant" → `k Q n s @ n @ n t`; final `t` is unreleased.
- "silent" → `s aI l @ n t`; "t" unreleased, natural silence follows.
- Every confessional line is half-whispered: PSOLA reduces F0
  consistency and voicing fraction. We want breath near the words.

## openSMILE gates
- voicing fraction: target 0.40, floor 0.30 (whispered confession).
- pitch error RMS ceiling: 5.0 cents (wider because whispered pitch
  is harder to track).
- loudness integrated: -22.0 LUFS (the quietest so far).
- spectral flux: maximum 0.02 per frame — confession must be steady.
