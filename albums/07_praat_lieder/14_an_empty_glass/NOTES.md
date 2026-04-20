# 14 An Empty Glass — notes

## Why this slot
The third and final `m.vowel_drone` reprise on the album. The drone here
is /u/ — the classic "singing-glass" vowel — held 10 seconds. We also
rhyme with 09 Kettle on Low (we sing back something the object cannot
sing itself), which tightens the back half of the album around a small
rhyme: we are repeatedly the voice that stands in for an object.

## Form and motif deployment
Through-composed, 18 bars in B phrygian at 50 bpm, 4/4. Two motifs:
`m.whisper_psola` across the verse, and `m.vowel_drone` on the held /u/
at the end. The drone centres on B3 (246.94 Hz), held 10 seconds. The
second phrygian-mode track of the back half, which partners with 04
Kitchen Scale's E phrygian — an internal album pairing.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts` for the
drone tuning; **`pitch_extract.praat`** verifies the held-/u/ F0 stays
within ±10 cents of B3 across the 10-second hold. Pitch extraction
autocorrelation (`ac`), floor 65 Hz, ceiling 1000 Hz, very-accurate on.
Target median 205 Hz for the verse, 246.94 Hz for the drone. Range
factor 0.66 — very tight. Formant shift ratio 1.02 — a tiny upward
shift that brightens the /u/ slightly so it reads as a glass-rim ring
rather than a vowel sigh.

## csdr graph shape
`shaped_noise_bed` at -28 dB, narrow band 400–1600 Hz — the same shape
as 05 Last Streetlamp and 09 Kettle on Low, because all three are the
"standing-in-for-an-object's-tone" tracks. Bed sustains through the
drone. No reverb.

## G2P / pronunciation hints
- "counter" as /'k aU n t @/ — two syllables.
- "fingertip" as /'f I N g @ t I p/ — three syllables, stress first.
- "circle" as /'s 3r k l=/ — syllabic /l=/.
- "uuuu" as /u:/ — this is the drone target, held 10 s at B3.

## openSMILE gate
Voicing fraction in [0.40, 0.60] — the long vowel raises it. Pitch-error
RMS 2 cents on verse; vowel-drone RMS ≤ 10 cents of B3. Loudness -18.
