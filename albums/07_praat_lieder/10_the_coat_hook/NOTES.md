# 10 The Coat Hook — notes

## Why this slot
We continue the slow middle of the album. The coat hook is the most
austere object in the cycle — single point of hardware, permanent, ordinary.
us3 returns (after 03 Station Clock) because the voice should again feel
a step removed; we do not want the intimacy of us1 for a piece about
an absence.

## Form and motif deployment
Through-composed, 22 bars in C# minor at 58 bpm, 3/4. C# minor is the
lowest key-centre on the album so far. The 3/4 meter keeps the pulse on
a waltz-shape without the lift a faster 3 would have. `m.whisper_psola`
across the whole track. No secondary motif.

## Praat treatment
Leans on **`formant_shift.praat`** from `nadir-praat-scripts`. Pitch
extraction autocorrelation (`ac`), floor 60 Hz, ceiling 900 Hz,
very-accurate on. Formant shift ratio 0.88 — the most downward shift on
the album; we want the voice slightly larger and darker, as though a
room's worth of space sits behind it. Target median 175 Hz. Range factor
0.72. Duration factor 1.08 — we stretch each syllable 8% beyond MBROLA's
natural timing. The extra breath-room matches the wear-mark-of-patience
image.

## csdr graph shape
`band_limit` preset at -26 dB, 180–2000 Hz. Bed silenced for bars 1–3
(we begin with the object) and bars 20–22 (we leave it). No reverb. No
ducking — we want a steady bed throughout the middle, so the vocal
whispers *into* something rather than floating.

## G2P / pronunciation hints
- "brass" as /b r & s/ — short, clean.
- "empty" as /'E m p t i/ — keep the /mp/ cluster tight.
- "patience" as /'p eI S @ n s/ — three syllables, stress first.
- "knuckle" as /'n V k l=/ — silent /k/ at the onset, syllabic /l=/.

## openSMILE gate
Voicing fraction in [0.28, 0.50] — us3 with a 0.88 formant shift sits
lower. Pitch-error RMS 2 cents ceiling. Loudness -18 LUFS integrated.
