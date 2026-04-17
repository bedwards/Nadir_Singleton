# 05 The Last Streetlamp — notes

## Why this slot
The midpoint of the first eight. We have accumulated enough rooms and
benches; we now step outside again and find a single light that has
refused to go out. The last line resolves the track onto a held /oU/ —
this is where we reprise `m.vowel_drone` from album 05 Phoneme Monastery.

## Form and motif deployment
Through-composed, 20 bars in G minor at 52 bpm, 6/8. The 6/8 meter gives
a slow lilt that fits the yellow-orange flicker of a sodium lamp. Two
motifs stack: `m.whisper_psola` across the whole track, and
`m.vowel_drone` on the final line. The drone is 8 seconds of held /oU/
with micro-vibrato ≤ 10 cents; its F0 centres on the tonic G3 (196 Hz).
The /oU/ drone is produced by pitching the MBROLA /oU/ token through
PSOLA onto a sustained target contour, then feeding the result of
`pitch_extract.praat` back to verify the micro-vibrato envelope.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts` for the
sung lines; **`pitch_extract.praat`** is invoked as a verification pass
to confirm the vowel-drone F0 stays within ±10 cents of G3 across the
8-second hold. Pitch extraction autocorrelation (`ac`), floor 65 Hz,
ceiling 950 Hz, very-accurate on. Target median 185 Hz for the verse
lines, 196 Hz for the final drone. Range factor 0.68. Formant shift
ratio 1.00.

## csdr graph shape
`shaped_noise_bed` at -28 dB, band 400–1600 Hz (narrower than usual) to
stand in for the sodium-lamp hum. The bed fades up across the first 4
bars and sustains under the final drone. No reverb. No other texture.

## G2P / pronunciation hints
- "filament" as /'f I l @ m @ n t/ — three syllables, stress on the first.
- "hum" as /h V m/ — do not nasalise through the /m/; let it close cleanly.
- "vowel" as /'v aU @ l/ — two syllables.
- "oh" as /oU/ — this is the drone target; must be held 8 s.

## openSMILE gate
Voicing fraction in [0.35, 0.60] (the drone raises it). Pitch-error RMS
2 cents ceiling across the verse; vowel-drone RMS ≤ 10 cents of G3.
Loudness -17 LUFS integrated.
