# 01 First Salt — notes

## Why this slot
We open the album here because the whole arc rests on a single first utterance. The singleton has not yet spoken; this track is the instant before language becomes habit. It must feel discovered, not composed.

## Form and motif deployment
Through-composed, 24 bars. The first 8 bars sit on a single A3, held under noise at -24 dB. At bar 9 we deploy `m.dawn_utterance` for the first time in the corpus: a rising minor third from A3 to C4, then plateau on C4 for four bars. The second half of the piece repeats this gesture transposed up a fourth (D4 → F4), answering itself. No cadence; we end on an unresolved C4.

## csdr graph shape
Single chain: `shaped_noise_dawn` into a linear-phase FIR band-limit 80–3.2 kHz, summed with the MBROLA vocal routed through the same band-limit so both sit in one spectral window. A slow `agc` ramp from silence over the first 3 seconds. No reverb; we want the dawn to be dry and near.

## G2P / pronunciation hints
- "salt" as /s O l t/ — hold the /O/ long; the /l/ should be dark.
- "morning" as two syllables /m O r/ + /n I N/; we do not want "morn-ning".
- "tongue" as /t V N/, not /t V N g/.
- Place a short pause after each line (the line breaks are breath points).

## openSMILE gate (primary)
Voicing fraction on the vocal stem greater than or equal to 0.7 across bars 9–24. Pitch-error RMS at 2 cents ceiling. Loudness gated at -14 LUFS integrated for the mix.
