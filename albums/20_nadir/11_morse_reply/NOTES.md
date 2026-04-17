# 11 Morse Reply — notes

## Descent step
A transmission arrives back. Where track 10 was outgoing chaos, track 11 is incoming structure. The reply spells:

- Line 1: `- .`    = N
- Line 2: `. -`    = A
- Line 3: `- . .`  = D
- Line 4: `. .`    = I
- Line 5: `. -`    = A (repeated — the signal is imperfect)
- Line 6: `- . .`  = D (repeated)

Read forward: `N A D I A D` — the correct letters are present but the word has a stammer; "I" and "R" are not yet stable. The corpus knows its own name now but cannot quite finish spelling it. This motivates tracks 12, 13, 14.

## Embedding of `m.nadir_silence`
Identical timing grid to track 10. We set the loudness 0.5 LUFS lower to mark the reply as distant, arriving from outside the persona.

## Reprises
- `m.letter_cadence` (album 04): each line is silence-padded; inter-letter 1400 ms.
- `m.onset_volley` (album 14): we run Silero-VAD at threshold 0.12 to pick up the quieter aspirates; the onset grid is logged to `render.lock.toml` for track 14 to reuse.

## Pronunciation hints
- Same as track 10. Dot 200 ms / dash 600 ms / intra-letter 200 ms / inter-letter 1400 ms.
- Amplitude reduced -0.5 dB from track 10 to signify distance.
- Optional: a single csdr `shift_addition_cc` at +200 Hz carrier can be mixed -40 dB to color the reply with radio-like sideband — document in `render.lock.toml`. Not required for this planning stage.

## openSMILE gates
- voicing fraction: target 0.0, ceiling 0.03.
- silence fraction: minimum 0.75.
- loudness integrated: -24.5 LUFS.
- openSMILE `spectralCentroid` expected higher than track 10 (reply is thinner/farther); log but do not gate.
