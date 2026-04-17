# 01 Matins — notes

## Why this slot
We open the album on the first office of the monastic day. The rule of
this cloister is that the day begins with a single held vowel broken out
of silence. `matins` is the instant when the brotherhood leaves sleep
and enters voice; the first /a/ must feel inevitable, not triggered.

## Form and motif deployment
Through-composed, 16 bars at 44 bpm (≈ 87 s). Bars 1–4 are silence
under a `dirac_impulse_bed` at 0.5 Hz — one slow click every two bars,
the tower bell before matins. Bar 5 places the first /a/ at A3
(220 Hz) held 8 s: this is the canonical `m.vowel_drone`. Bar 9 inherits
the reprise `m.dawn_utterance` — rising minor third A3 → C4 under the
same held /a/ (formant shift, not new attack). The final bar decays
back to the bell alone.

## csdr graph shape
`dirac_impulse_bed 0.5` summed with vocal routed through a
`fir_cascade` of three bandpasses tuned to the first three formants of
/a/ at us3 (F1 ≈ 720, F2 ≈ 1240, F3 ≈ 2500 Hz, all normalized against
48 kHz). The cascade accentuates the open vowel without coloring it
toward any other phoneme. No reverb; we rely on the slow impulse bed
to suggest stone.

## G2P / pronunciation hints
Lyric is pure /a/ repeated. MBROLA voice `us3` pronounces ASCII `a` as
SAMPA `a` — the open front unlaxed vowel. Each occurrence is held
7–9 s with 50 cent micro-vibrato (Praat PSOLA modulation at 4 Hz
depth 0.3 semitones). Silence between occurrences is 2–4 bars.

## openSMILE gates
- voicing fraction: target 0.55, floor 0.45 (majority of window is
  silence between sustained drones).
- pitch error RMS ceiling: 3.0 cents.
- loudness integrated: -20.0 LUFS.
- F0 stddev semitones within each /a/: maximum 0.4 ST.
