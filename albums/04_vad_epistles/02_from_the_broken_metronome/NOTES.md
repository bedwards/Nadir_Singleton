# 02 From the Broken Metronome — notes

## Why this slot
A reply letter, from the metronome of album 02 back to us. Letters work
in both directions; we establish that here. The dorian mode gives the
reply a different weather than the G major thank-you that preceded it.

## Form and motif deployment
Through-composed, 24 bars in D dorian at 72 bpm. `m.letter_cadence` on
"wobble" — 1600 ms pad. The tail is longer than track 01 because the
metronome is still thinking even after it has signed off.

## reprises
- The Broken Metronome (`02_tin_pan_fathom/04_the_broken_metronome`) — the
  writer of this letter. We keep D as home key but swap major for dorian
  so the reply feels reflective rather than jaunty.
- "the cat" is the friendly-old-cat from track 04 of album 02.
- "the piano" is the upright from `16_old_piano_in_the_hall`.

## csdr graph shape
`shaped_noise_bed` again, but with a faint 2.4 Hz amplitude wobble applied
via csdr `dsb_fc` — a quiet nod to the metronome's wander. The wobble sits
at -32 dB so it is felt more than heard. Band-limit 100-2200 Hz.

## G2P / pronunciation hints
- "mantel" as /'m & n t @ l/ — we used this in album 02; keep it identical.
- "wobble" as /'w A b @ l/ — soft /b/, open vowel; final-syllable hold is
  on the /@ l/ — let the schwa decay.
- "piano" as /p i '& n oU/ — three syllables.
- "sincerely" as /s I n 's I r l i/ — avoid /rl/ cluster by leaning
  into the /I r/ as a single nucleus.
- Avoid "broken" 's /kn/ temptation: /'b r oU k @ n/.

## Silero-VAD wiring
Threshold 0.35. Letter-ending silence measured off the fall-edge of the
segment containing "wobble". 1600 ms pad. The listener should feel the
envelope being closed slowly.

## openSMILE gate
Voicing fraction >= 0.55 — the metronome's voice breathes even more than
ours. Pitch-error RMS 2 cents. -16 LUFS integrated.
