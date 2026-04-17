# 08 The Unopened Letter — notes

## Why this slot
Album 04 VAD Epistles was a cycle of letters we wrote and sent; this
Lied is a letter we received and did not open. It is the inverse of the
04 opening. We reprise `m.letter_cadence` here — the silence-padded final
syllable — but stretch it to 2200 ms (longer than album 04's 1400 ms)
because no reply is coming.

## Form and motif deployment
Through-composed, 20 bars in B minor at 52 bpm, 4/4. Two motifs:
`m.whisper_psola` across the whole track, and `m.letter_cadence` on
the final word "keeping". The VAD fall-edge on that syllable must be
clean; the 2200 ms of silence after is the envelope the letter sits in.
Per-phoneme durations are gently warped via `duration_warp.praat` so
the final line's rhythm matches the letter_cadence template exactly.

## Praat treatment
Leans on **`duration_warp.praat`** from `nadir-praat-scripts` for the
per-phoneme stretching that produces the letter_cadence tail, with
**`psola_retarget.praat`** called in sequence for the pitch flattening.
Pitch extraction autocorrelation (`ac`), floor 65 Hz, ceiling 950 Hz,
very-accurate on. Target median 195 Hz. Range factor 0.70. Formant shift
ratio 1.00.

## csdr graph shape
`shaped_noise_bed` at -28 dB, narrower band 250–1700 Hz. Bed is gated
out entirely for the last 3 bars and during the 2200 ms of silence — the
silence is truly silent, not noise-floor. No reverb.

## Silero-VAD wiring
Threshold 0.35. `min_speech_duration_ms = 120`,
`min_silence_duration_ms = 80`. Each line-break is a breath point; the
VAD confirms segments break at line endings. The 2200 ms tail is
measured from the fall-edge of the final segment on "keeping".

## G2P / pronunciation hints
- "tuesday" as /'t u z d eI/ — two syllables; clean /z/.
- "handwriting" as /'h & n d r aI t I N/ — four syllables, stress first.
- "keeping" as /'k i p I N/ — long /i/, soft /N/. This is the cadence
  word; treat it like the word "dawn" in album 04 track 01.

## openSMILE gate
Voicing fraction in [0.30, 0.50]. Pitch-error RMS 2 cents ceiling.
Loudness -18 LUFS integrated. Final-silence ms = 2200 ± 50.
