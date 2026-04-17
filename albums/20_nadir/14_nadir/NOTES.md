# 14 Nadir — notes

## Descent step
The final track. No words. No pitch. Only the five Morse letters of N-A-D-I-R, rendered as calibrated silences and aspirated `/h/` consonants. Track 14 is the single point at which the palette motif `m.nadir_silence` is executed in full, with every prior track having rehearsed one of its components.

## The Morse encoding (definitive spec)

International Morse code, letters of "NADIR":

| Letter | Morse  |
|--------|--------|
| N      | `- .`  |
| A      | `. -`  |
| D      | `- . .`|
| I      | `. .`  |
| R      | `. - .`|

### Timing grid (milliseconds)

| Unit                 | Duration | Representation    |
|----------------------|----------|-------------------|
| Dot `.`              | 200      | `[h]` aspirate    |
| Dash `-`             | 600      | `[hh]` aspirate   |
| Intra-letter gap     | 200      | pure silence      |
| Inter-letter gap     | 1400     | pure silence (`.` in `lyric.txt`) |
| Inter-word gap       | 2800     | pure silence (not used — one word)|
| Head / tail pad      | 1400     | pure silence      |

### MBROLA `.pho` skeleton

The render pipeline must emit (durations in ms; `_` is the MBROLA silence phoneme):

```
_ 1400                    ; head pad
h 600 ; _ 200 ; h 200     ; N = dash dot
_ 1400                    ; inter-letter
h 200 ; _ 200 ; h 600     ; A = dot dash
_ 1400
h 600 ; _ 200 ; h 200 ; _ 200 ; h 200   ; D = dash dot dot
_ 1400
h 200 ; _ 200 ; h 200     ; I = dot dot
_ 1400
h 200 ; _ 200 ; h 600 ; _ 200 ; h 200   ; R = dot dash dot
_ 1400                    ; tail pad
```

### Total duration check
- Head: 1400
- N: 600 + 200 + 200 = 1000
- Gap: 1400
- A: 200 + 200 + 600 = 1000
- Gap: 1400
- D: 600 + 200 + 200 + 200 + 200 = 1400
- Gap: 1400
- I: 200 + 200 + 200 = 600
- Gap: 1400
- R: 200 + 200 + 600 + 200 + 200 = 1400
- Tail: 1400

Total = 1400 + 1000 + 1400 + 1000 + 1400 + 1400 + 1400 + 600 + 1400 + 1400 + 1400 = 13800 ms ≈ 13.8 s.

At 40 bpm (1500 ms per beat) in 4/4 that is 9.2 beats ≈ 2.3 bars of pure signal. The manifest allocates 12 bars to absorb DAW-side pre-roll and the final decay of the bed, which is faded out over 30 s starting at the tail-pad boundary.

## Embedding of `m.nadir_silence`
This IS `m.nadir_silence`. Every prior embedding in tracks 01–13 has rehearsed a subset:
- 01 introduced the 1400 ms inter-letter gap.
- 02 monotonically lengthened silences to the 1400 ms standard.
- 03 established the 600 ms dash width.
- 06 rehearsed the 700 ms empty beat (close to dash).
- 08 committed 600 ms and 1400 ms gaps as hymn structure.
- 09 introduced the `[h]` / `[hh]` aspirate units with no voicing.
- 10 and 11 committed the full dot/dash/intra/inter grid but spelling other words.
- 12 isolated the letter N.
- 13 spelled the transmitter ("we").

Track 14 fuses every component into the word the album was always spelling: its own title.

## Reprises
- `m.vowel_drone` (album 05): absent as a vowel, present as its inversion — a consonantal drone of unvoiced breath.
- `m.letter_cadence` (album 04): silence-padded letter endings are now the whole form.
- `m.singleton_spine` (album 19): the persona itself, singular, named.
- `m.drift_delta` (album 12): zero drift — pitch is undefined, so pitch cannot drift. The 0.2 cents/bar of album 12 has decayed to 0 cents/bar. This is the end state.
- `m.dawn_utterance` (album 01): the corpus opened with a rising-3rd-then-plateau utterance; it closes with the absence of utterance. The symmetry is deliberate.

## Pronunciation hints
- Every `h` is MBROLA us3 `h` with stated duration, no F0 points (F0 column empty → MBROLA emits unvoiced aspirate).
- Intensity envelope: `h 200` at -22 dB RMS; `h 600` at -19 dB RMS. The dash is slightly louder so listeners unfamiliar with Morse still perceive the binary.
- Silences must be true digital silence (-inf dB) in the vocal stem. The csdr bed (`shaped_noise_nadir` preset) runs underneath at -54 dB integrated, low-passed at 180 Hz, so the track is not absolutely silent — there is a room tone, but the signal is purely silence and breath.
- The final 1400 ms tail pad transitions via a 30 s linear fade of the bed to true zero. The listener is left in silence beyond the end of the recording.

## openSMILE gates
- voicing fraction (eGeMAPSv02 `VoicedSegmentsPerSec`): target 0.0, ceiling 0.02. If openSMILE detects any voiced frames, the render is rejected.
- silence fraction (derived: 1.0 - voicing - aspirate): minimum 0.60.
- aspirate fraction (custom: `spectralFlatness_sma3_amean > 0.55` AND `loudness_sma3_amean > -30 dB`): target 0.15 ± 0.02.
- loudness integrated: -26.0 LUFS. This is the quietest track of the corpus.
- Silero-VAD onset count: exactly 12 (N=2, A=2, D=3, I=2, R=3). Any other count triggers a regen.
- openSMILE extraction is REQUIRED to certify the absence of voicing — the tool must appear in `render.lock.toml` even though it reports near-zero.

## Reproducibility
Track 14 is the most constrained render in the corpus. The `render.lock.toml` must pin MBROLA version, the exact `.pho` bytes (SHA-256), the csdr bed graph, and the Silero-VAD onset timestamps. Future renders that diverge by > 10 ms on any onset are considered non-conforming.
