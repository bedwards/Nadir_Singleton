# Praat Lieder

_Album 7 of 20 in the Nadir_Singleton corpus._

Sixteen art songs, one voice, minimal bed; PSOLA is the composer.

## Motif palette
minimal art-song; `m.whisper_psola` (Praat PSOLA de-voicing of MBROLA) debuts
and reprises in almost every track. We reach back for `m.vowel_drone`
(album 05) and `m.letter_cadence` (album 04) where the small object asks for
breath or silence.

## Arc
Each Lied is a small room built around one tiny thing: a window, a glove,
a station clock, a kitchen scale, the last streetlamp, a dictionary entry,
a spoon of salt, an unopened letter, a kettle on low, a coat hook, reading
glasses, a moth at the lamp, a thimble, an empty glass, a folded map, the
small hours. We move from first light (01) to the hour before first light
(16) — the cycle closes by almost opening again.

PSOLA is the composer in this album because every vocal is MBROLA material
re-pitched, de-voiced, or formant-shifted by Praat. The bed is either
silence or a single band-limited noise shape at -24 dB. No csdr beds run
louder than -24 dB. No ring modulation. No granular textures. Nothing
competes with the voice.

## Tracks (planned)
_16 tracks. Per-track `manifest.toml`, `lyric.txt`, `NOTES.md` populated._

```
01  A Window                        F# minor         m.whisper_psola
02  A Glove                         D dorian         m.whisper_psola
03  Station Clock                   A minor          m.whisper_psola
04  Kitchen Scale                   E phrygian       m.whisper_psola
05  The Last Streetlamp             G minor          m.whisper_psola, m.vowel_drone
06  Dictionary Entry                C major          m.whisper_psola
07  A Spoon of Salt                 A minor          m.whisper_psola, m.dawn_utterance
08  The Unopened Letter             B minor          m.whisper_psola, m.letter_cadence
09  A Kettle on Low                 D minor          m.whisper_psola, m.vowel_drone
10  The Coat Hook                   C# minor         m.whisper_psola
11  Reading Glasses                 F major          m.whisper_psola
12  The Moth and the Lamp           E minor          m.whisper_psola
13  A Thimble                       G# phrygian      m.whisper_psola
14  An Empty Glass                  B phrygian       m.whisper_psola, m.vowel_drone
15  The Folded Map                  D dorian         m.whisper_psola
16  The Small Hours                 F# minor         m.whisper_psola, m.letter_cadence
```

## Production notes
Every track uses the `nadir-praat-scripts` library explicitly. The per-track
NOTES.md names which `.praat` script each song leans on. PSOLA pitch
extraction defaults to autocorrelation (`ac`) at floor 65 Hz, ceiling 1000
Hz, very-accurate on; a few tracks switch to `cc` (for formant-adjacent
partials) or `shs` (for intentionally degraded intimacy). Formant-shift
ratios stay inside 0.85–1.18 — art-song, not cabaret.

Bed presets are only `band_limit` (silence-adjacent) or `shaped_noise_bed`,
both capped at -24 dB. Several tracks run pure silence under the voice.

Keys lean minor, dorian, and phrygian for art-song weight; two tracks
(06 Dictionary Entry, 11 Reading Glasses) take major for contrast — these
are the two pieces where the voice briefly smiles.

us1 is our default MBROLA voice (warm, closer mic); us3 appears where the
object wants a thinner, further-off reading (03, 10, 15). Every track
is 8–20 lyric lines.
