# 09 A Kettle on Low — notes

## Why this slot
The second `m.vowel_drone` reprise in the album, longer than 05's. This
one is the tonic-length drone — we hold /a/ for 12 seconds, matching the
album 05 Phoneme Monastery canonical vowel-drone duration. The kitchen
returns as a location (04 Kitchen Scale was here too), but now we are
making sound rather than weighing flour.

## Form and motif deployment
Through-composed, 16 bars in D minor at 48 bpm, 4/4. The slowest tempo on
the album. Two motifs: `m.whisper_psola` across the verse, and
`m.vowel_drone` on the final two lines. The drone is on /a/ (SAMPA
/A:/ held) centred on D3 (146.8 Hz) with micro-vibrato ≤ 10 cents. Held
12 seconds — we want an almost-too-long held vowel, uncomfortable only
for a moment, then settling.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts` for the
drone tuning; **`pitch_extract.praat`** verifies the held-/a/ F0 stays
within ±10 cents of D3 for 12 seconds. Pitch extraction autocorrelation
(`ac`), floor 65 Hz, ceiling 1000 Hz, very-accurate on. Target median
185 Hz for the verse, 146.8 Hz for the drone. Range factor 0.62 — very
tight. Formant shift ratio 0.98 — a tiny downward shift to darken the
vowels in sympathy with the kettle's hum.

## csdr graph shape
`shaped_noise_bed` at -26 dB, narrow band 350–1400 Hz to stand in for
the kettle's own ambient shape (not rendering the whistle — the whistle
is implied, the voice is the whistle). The bed holds underneath the
12-second vowel-drone at the same level. No reverb.

## G2P / pronunciation hints
- "kettle" as /'k E t l=/ — syllabic /l=/.
- "thread" as /T r E d/ — voiced final /d/, slight theta attack.
- "argument" as /'A r g j u m @ n t/ — four syllables.
- "ah" as /A:/ — the long open /a/, this is the drone target.

## openSMILE gate
Voicing fraction in [0.40, 0.65] — the long vowel raises it. Pitch-error
RMS 2 cents on verse; vowel-drone RMS ≤ 10 cents of D3 across the 12 s
hold. Loudness -18 LUFS integrated.
