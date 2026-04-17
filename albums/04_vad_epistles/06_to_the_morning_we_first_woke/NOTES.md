# 06 To the Morning We First Woke — notes

## Why this slot
Midway through the album we write to the dawn itself. This is the most
direct reprise of album 01 and we let the `m.dawn_utterance` shape — a
rising minor third — sit under the melody of the first two lines without
quoting it outright. The rest of the song departs from that gesture so
we do not turn the letter into a cover song.

## Form and motif deployment
Through-composed, 18 bars in A minor at 60 bpm. `m.letter_cadence` on
"lit" — 1800 ms pad. The silence after "lit" is the longest in the first
half of the album; we hold the room open as long as a listener might
reasonably wait.

## reprises
- First Salt (`01_horizon_salts/01_first_salt`) — "salt on the tongue"
  is a near-direct quotation. A minor key is the same.
- The Horizon Appears (`01_horizon_salts/08_the_horizon_appears`) — the
  "small window" image echoes its opening.
- Shape (not pitch) of `m.dawn_utterance` under the first two lines.

## csdr graph shape
`shaped_noise_bed` with the shaped noise pushed toward the 80-3200 Hz
band (the dawn bed of album 01) but at -30 dB — a memory of that bed,
not the bed itself. Vocal sits at album-01 tessitura floor. An agc ramp
over the first 2 s to re-enact the waking. No reverb.

## G2P / pronunciation hints
- "morning" as /'m O r n I N/ — identical to album 01's treatment.
- "salt" as /s O l t/.
- "tongue" as /t V N/.
- "handful" as /'h & n d f @ l/ — four phonemes at the /nd f/ juncture;
  keep them clean.
- "letter" as /'l E t @/ — no /r/ in final syllable for us1.
- "lit" as /l I t/ — final-syllable hold is the /t/ release.

## Silero-VAD wiring
Threshold 0.35. We deliberately expect longer inter-line silences here
(the writer is pausing to remember). `min_silence_duration_ms = 120`
rather than 80 so mid-line breath points do not become segment breaks.

## openSMILE gate
Voicing fraction >= 0.55. Pitch-error RMS 2 cents. -16 LUFS integrated.
