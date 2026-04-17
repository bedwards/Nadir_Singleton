# 03 Lauds — notes

## Why this slot
Lauds is the dawn praise; the monastery's rule admits /o/ at this hour
as a second voice next to the /a/ of matins. We move to C dorian to
brighten the modal color without breaking the drone tradition (dorian's
natural 6 supports the /o/ formant in a way aeolian would dull). The
voice switches to `us1` — a younger cantor rising to meet the light.

## Form and motif deployment
AABA over 18 bars at 60 bpm. Each A holds /o/ on C4 for 8 s as the
canonical `m.vowel_drone`. The B section (bars 9–12) calls
`m.dawn_utterance` — rising minor third C4 → Eb4 (dorian #3 substituted
briefly with the minor third for antiphon color) on the /a/ that
"answers". This is the first dialogue of the album — two vowels, two
registers, one call-and-response. Final A resumes /o/; a short coda
"la la lo / la la lo" is the first pseudo-syllable ritornello.

## csdr graph shape
`fir_cascade` tuned to /o/'s F1/F2 (≈ 510, 870 Hz) for the drone, and a
`dirac_impulse_bed` at 1.0 Hz — the tower bell has quickened from matins'
0.5 Hz. The quickened pulse is the structural marker of daylight
arrival.

## G2P / pronunciation hints
- "o" → SAMPA `O` for us1 (open-mid back rounded); held 7 s with 4 Hz
  micro-vibrato, 0.3 ST depth.
- "morning" → `m O r n I N`; the opening `m O` hinges into the drone
  seamlessly.
- "answers" → `a n s @ z`; first-syllable `a` is the albumic open /a/
  held 2 s, resolving the `m.dawn_utterance` rise.
- "la la lo" → `l a l a l @U`; rhythmic but still legato. Do not
  articulate a hard `l`; leave it soft so the vowel is primary.

## openSMILE gates
- voicing fraction: target 0.75, floor 0.65.
- pitch error RMS ceiling: 3.0 cents.
- loudness integrated: -20.0 LUFS.
- spectral centroid Hz: target 650 ± 150 (keeps the /o/ dominant).
