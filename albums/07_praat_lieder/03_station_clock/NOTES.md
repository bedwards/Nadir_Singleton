# 03 Station Clock — notes

## Why this slot
A public object we are not supposed to be alone with. The station clock is
a sound as much as a shape — we put us3 (thinner, further-off voice) on
this track specifically because us1 would sound too intimate. The mood is
suspended time.

## Form and motif deployment
Through-composed, 16 bars in A minor at 50 bpm, 4/4. Very slow. The
melodic contour barely moves — most syllables sit within a minor third
of each other, which is a deliberate PSOLA range compression. This is
the track that establishes `m.whisper_psola` as capable of carrying a full
song on pitch flattening alone.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts`. Pitch
extraction cross-correlation (`cc`) — we pick CC over AC here because
MBROLA's us3 output has a slightly harmonic-rich spectrum on the schwas
that confuses AC; CC's robustness to formant-adjacent partials gives
cleaner pulse detection. Floor 60 Hz, ceiling 900 Hz, very-accurate on.
Target median 180 Hz (low for us3). Range factor 0.60 — the tightest
compression in the album. Formant shift ratio 1.00.

## csdr graph shape
`shaped_noise_bed` at -28 dB, deliberately narrower band 300–1800 Hz,
shaped to simulate the ambient air of a large covered platform without
rendering it. Bed gated off entirely for the final 4 bars (bars 13–16)
so the last line lands on silence. No reverb; the reverberance is implied
by the bed spectrum, not added.

## G2P / pronunciation hints
- "numerals" as /'n u m @ r @ l z/ — keep the unstressed vowels as schwas.
- "roman" as /'r oU m @ n/ — clean, no aspiration on the /r/.
- "announcement" as /@ 'n aU n s m @ n t/ — stress on the second syllable.
- "click" appears twice; match the /k l I k/ attack exactly both times.

## openSMILE gate
Voicing fraction in [0.30, 0.45]. Pitch-error RMS 2 cents ceiling.
Loudness -18 LUFS integrated.
