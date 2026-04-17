# 04 Terce — notes

## Why this slot
Terce, the third hour, admits the third vowel. /u/ joins /a/ and /o/.
The brotherhood is complete at this office. We chose D phrygian: the
lowered second (Eb) creates the narrow semitone approach that evokes
ancient plainchant, and the dorian→phrygian move from lauds is a small
color shift appropriate for mid-morning darkening. 6/4 meter gives six
slow pulses per bar — three vowels, two breaths apiece.

## Form and motif deployment
Through-composed, 12 bars at 58 bpm. Bars 1–3 each introduce one vowel
held 7 s: first /a/ (A3), then /o/ (D4, tonic), then /u/ (A3 again, one
octave low). Bar 4–5 braid all three as `a o u` — the canonical phrygian
trichord as vowels. Bars 6–7 the /u/ sustains alone for 10 s — the
darkest drone of the album so far. Bars 8–10 repeat `a o u` three times
rhythmically on the same pitch centre. Final bar reverses the order:
`u o a`, the brotherhood returning toward /a/. Every long held vowel
is `m.vowel_drone`; the triad `a o u` is the album's structural motto.

## csdr graph shape
Three `fir_cascade` presets, one per vowel, switched by which vowel is
current. /a/: bands at 720/1240/2500 Hz. /o/: 510/870/2500 Hz. /u/:
320/780/2200 Hz (all normalized against 48 kHz). `dirac_impulse_bed` at
0.5 Hz returns from matins — the tower bell slows mid-morning as the
community settles.

## G2P / pronunciation hints
- `a` → SAMPA `a` for us3; 7 s hold, 4 Hz vibrato at 0.3 ST.
- `o` → SAMPA `O`; 7 s hold.
- `u` → SAMPA `u:` (long close back rounded); 7 s first occurrence,
  10 s in bars 6–7.
- Each vowel transition is a glide, not a re-attack: Praat PSOLA morphs
  formants over 600 ms between vowels. The listener must not hear three
  separate voices but one voice moving through three shapes.

## openSMILE gates
- voicing fraction: target 0.80, floor 0.70.
- pitch error RMS ceiling: 3.5 cents.
- loudness integrated: -20.0 LUFS.
- F0 stddev semitones: maximum 0.5 ST across any held vowel.
