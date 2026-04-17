# 13 A Thimble — notes

## Why this slot
Inherited object. The smallest physical object in the cycle, and the
oldest. We move the Lied into G# phrygian — the phrygian's half-step
tonic-to-supertonic motion matches the small-press gesture of a thimble
against cloth. This is one of two phrygian tracks (with 04 Kitchen
Scale); both are small-domestic-ritual objects.

## Form and motif deployment
Through-composed, 18 bars in G# phrygian at 54 bpm, 3/4. `m.whisper_psola`
across the track. No secondary motif. The melodic figure descends on
the final line ("longer than we have"), ending on the tonic G#3 — a
quiet, small cadence that matches the thimble's shape.

## Praat treatment
Leans on **`formant_shift.praat`** from `nadir-praat-scripts`. Pitch
extraction autocorrelation (`ac`), floor 70 Hz, ceiling 1050 Hz,
very-accurate on. Formant shift ratio 1.12 — the second-highest upward
shift on the album (after 11's 1.08 — wait, 1.12 is higher, intentional).
The shift brightens the vowels to match silver's ring. Target median
225 Hz. Range factor 0.76. Duration factor 0.95 — we shorten each
syllable 5% so the meter feels slightly clipped, like small metal
pushing through fabric.

## csdr graph shape
`band_limit` preset at -24 dB, 280–2600 Hz. The slightly raised lower
cutoff removes the low-body warmth that this Lied does not need — silver,
not wool. Bed silenced for bars 14–18 (the resting-in-the-drawer closing
image). No reverb.

## G2P / pronunciation hints
- "thimble" as /'T I m b l=/ — theta attack, syllabic /l=/.
- "inherited" as /I n 'h E r I t I d/ — four syllables, stress on 'h E r'.
- "pushings" as /'p U S I N z/ — the /U/ vowel, voiced final /z/.
- "scissors" as /'s I z @ z/ — three syllables, voiced middle and final.

## openSMILE gate
Voicing fraction in [0.35, 0.55]. Pitch-error RMS 2 cents ceiling.
Loudness -18 LUFS integrated.
