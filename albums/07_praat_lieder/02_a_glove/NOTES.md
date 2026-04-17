# 02 A Glove — notes

## Why this slot
Track 01 put us in a room looking at a window; track 02 takes us outside
and sits us on a bench. A single leather glove, unattended. We do not
narrate ownership or loss — the object is enough. This is the first track
where the PSOLA treatment also touches formants (track 01 held formants
neutral).

## Form and motif deployment
Through-composed, 20 bars in D dorian at 60 bpm, 4/4. Dorian leans the
melody melancholic-but-not-heavy, which suits the refusal to dramatise.
`m.whisper_psola` again across the whole track; the voicing reduction is
applied post-PSOLA by scaling voicing amplitude toward 0 in the Manipulation
object before resynthesis.

## Praat treatment
Leans on **`formant_shift.praat`** from `nadir-praat-scripts`. Pitch
extraction autocorrelation (`ac`), floor 65 Hz, ceiling 900 Hz,
very-accurate on. Formant shift ratio 0.92 — a small downward shift to
make the voice feel slightly further-off than track 01 (a body sitting on
the bench next to the listener, not at their shoulder). Target median
195 Hz. Range factor 0.80. Duration factor 1.05 — we let every syllable
stretch 5% beyond MBROLA's natural timing so the meter settles.

## csdr graph shape
`band_limit` preset at -26 dB, band 180–2100 Hz. Silence under the voice
for bars 1–4 and 16–20; the bed only appears in the middle where the
inventory of the glove is most concrete. No reverb.

## G2P / pronunciation hints
- "glove" as /g l V v/ — do not devoice the final /v/ to /f/.
- "leather" as /'l E D @/ — the /D/ (eth) must be voiced and soft.
- "curled" as /k 3r l d/ — full /r/ colouring.
- "colder" and "warmer" on successive lines — keep the /r/s matched so
  the opposition lands.

## openSMILE gate
Voicing fraction in [0.30, 0.50]. Pitch-error RMS 2 cents ceiling.
Loudness -18 LUFS integrated.
