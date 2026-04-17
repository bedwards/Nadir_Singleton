# 16 The Small Hours — notes

## Why this slot
Closing Lied. The hour before first light — the album began at a window
at dawn (01), and it ends at a window that is about to decide what the
morning will be. The cycle almost opens again. Every object from the
previous fifteen tracks is named once more, in order — a small roll-call
inside the closing song. The final line "we will agree again" deliberately
rhymes with track 01's closing "and we agree".

## Form and motif deployment
Through-composed, 24 bars in F# minor at 48 bpm, 3/4. Same key as track
01 A Window — the album closes where it opened, a fifth-lower in energy.
Two motifs: `m.whisper_psola` across the track, and `m.letter_cadence`
on the very final word "again". The cadence tail is 3000 ms of silence,
the longest on the album — longer than 08 The Unopened Letter's 2200 ms,
because this is not a letter we are waiting on; this is the cycle
closing itself.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts`. Pitch
extraction autocorrelation (`ac`), floor 65 Hz, ceiling 1000 Hz,
very-accurate on. Target median 210 Hz. Range factor 0.72. Formant shift
ratio 1.00 — we end the album without formant manipulation, the same
way we began it (track 01 was also ratio 1.00). Symmetric bookends.

## csdr graph shape
`band_limit` preset at -26 dB, 200–2200 Hz. The roll-call bars (5–19)
run the bed at a steady -26 dB. Bars 20–24 and the 3000 ms tail are
pure silence — no band-limit noise, no reverberance, nothing. The last
thing the album says is silence.

## Silero-VAD wiring
Threshold 0.35. `min_speech_duration_ms = 120`,
`min_silence_duration_ms = 80`. The 3000 ms tail is measured from the
fall-edge of the final segment on "again". The final silence is the
letter_cadence motif at its album-closing maximum length.

## G2P / pronunciation hints
- For the roll-call, match each earlier track's pronunciation exactly:
  window /'w I n d oU/ (01), glove /g l V v/ (02), clock /k l A k/ (03),
  scale /s k eI l/ (04), lamp /l & m p/ (05), page /p eI dZ/ (06),
  spoon /s p u n/ (07), letter /'l E t @/ (08), kettle /'k E t l=/ (09),
  hook /h U k/ (10), glasses /'g l & s I z/ (11), moth /m A T/ (12),
  thimble /'T I m b l=/ (13), glass /g l & s/ (14), map /m & p/ (15).
- "again" as /@ 'g E n/ — this is the cadence word; match track 01's
  "agree" tail weight.

## openSMILE gate
Voicing fraction in [0.30, 0.50]. Pitch-error RMS 2 cents ceiling.
Loudness -19 LUFS integrated (quietest on the album, by design).
Final-silence ms = 3000 ± 50.
