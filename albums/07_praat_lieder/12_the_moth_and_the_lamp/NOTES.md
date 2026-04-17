# 12 The Moth and the Lamp — notes

## Why this slot
The emotional fulcrum of the back half. We paired ourselves with the
moth. The lamp is the same streetlamp logic as 05 (filament, held note)
but indoor and smaller. We use SHS pitch detection here — the only SHS
track on the album — because we deliberately want a slightly less
precise contour on the sustained /I/ and /i/ vowels of "ticking" and
"circling"; SHS is less precise in time, which reads as a tiny wing-beat
in the voice.

## Form and motif deployment
Through-composed, 20 bars in E minor at 62 bpm, 4/4. `m.whisper_psola`
across the track. No secondary motif; this Lied carries its weight on
pitch flattening alone.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts`, but with
pitch extraction via **subharmonic summation (`shs`)** rather than AC —
this is the album's one SHS track, chosen so the time-axis imprecision
of SHS produces a slight jitter in the sustained vowels that matches the
moth-wing tick. Floor 60 Hz, ceiling 900 Hz, very-accurate on. Target
median 200 Hz. Range factor 0.70. Formant shift ratio 0.95 — a small
downward shift for a slightly darker, dustier vowel.

## csdr graph shape
`shaped_noise_bed` at -27 dB, band 300–1800 Hz. We shape the bed to
carry a very faint irregular amplitude modulation at ~6 Hz (implemented
in `shaped_noise_bed.toml` via a gain_ff stage with a slow envelope
input) to stand in for the wing-tick without rendering it. No reverb.

## G2P / pronunciation hints
- "moth" as /m A T/ — short, open; the final /T/ must be audible.
- "shade" as /S eI d/ — single stressed syllable.
- "filament" as /'f I l @ m @ n t/ — match the pronunciation from 05
  Last Streetlamp; this is a cross-reference.
- "circling" as /'s 3r k l I N/ — the /3r/ vowel must carry; this word
  sits on the SHS jitter.

## openSMILE gate
Voicing fraction in [0.32, 0.52]. Pitch-error RMS 3 cents ceiling
(relaxed from 2 cents because SHS is less precise). Loudness -18 LUFS.
