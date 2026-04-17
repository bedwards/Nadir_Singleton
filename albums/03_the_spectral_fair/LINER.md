# The Spectral Fair

_Album 3 of 20 in the Nadir_Singleton corpus._

a fairground of feature vectors: each attraction a functional family.

## Motif palette
carnival eGeMAPS vectors

## Signature motif
`m.feature_vector_waltz` — swung 3/4, each beat one eGeMAPS family. Occasional
6/8 for the hall-of-mirrors, the fortune teller, and the hawkers.

## Tracks (planned)

| # | Slot | Family | Meter |
|---|------|--------|-------|
| 01 | Gates of the Fair | F0 mean / median | 3/4 |
| 02 | Loudness Barker | Zwicker loudness | 3/4 |
| 03 | The Spectral Slope Swing | slope0-500, slope500-1500 | 3/4 |
| 04 | HNR Tent | HNR | 3/4 |
| 05 | Jitter Hall of Mirrors | jitter local | 6/8 |
| 06 | Shimmer Carousel | shimmer local dB | 3/4 |
| 07 | MFCC Menagerie | MFCC 1-4 | 3/4 |
| 08 | Spectral Flux Flume | spectral flux | 3/4 |
| 09 | Formant Fortune Teller | F1, F2, F3 frequencies | 6/8 |
| 10 | Voicing Probability Ring | voicingFinalUnclipped | 3/4 |
| 11 | Alpha Ratio Aerialists | alphaRatio | 3/4 |
| 12 | Hammarberg High Striker | hammarbergIndex | 3/4 |
| 13 | H1-H2 Hawkers | logRelF0-H1-H2 | 6/8 |
| 14 | Bandwidth Booth | F1/F2/F3 bandwidths | 3/4 |
| 15 | Pitch Percentile Parade | F0 percentiles 20/50/80 | 3/4 |
| 16 | Midway Closing Lights | all families, coda | 3/4 |

## Production notes

We keep the waltz dominant across the album and use 6/8 only where the family
itself asks for mirroring or pairing (jitter's self-mirror, formants as two
cards per half-bar, hawkers as stalls on opposite sides). Each track's NOTES.md
names the openSMILE gates specific to that family; the album's global gates
are the standard Nadir_Singleton thresholds (F0 RMS < 2 cents, integrated LUFS
-14, MFCC1 delta < 0.3).

Cross-album motif visits:
- `m.dawn_utterance` (from album 01) in tracks 01 and 16 as a handshake and a
  farewell.
- `m.whisper_psola` (belongs to album 07) foreshadowed briefly in track 04's
  B section, 8 bars of de-voiced MBROLA.
