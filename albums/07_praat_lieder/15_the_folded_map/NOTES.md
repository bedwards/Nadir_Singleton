# 15 The Folded Map — notes

## Why this slot
The penultimate track gathers small objects from earlier in the cycle —
the thimble (track 13), implicitly the drawer, and a new addition (the
key). The folded-wrong map is the metaphor the album has been quietly
building: paper that remembers what we cannot. us3 returns (03, 10, 15 —
all three tracks with a slight physical remove).

## Form and motif deployment
Through-composed, 18 bars in D dorian at 58 bpm, 4/4. D dorian is the
same mode/centre as track 02 A Glove, deliberately — we open and close
the objects the album holds under the same modal roof. `m.whisper_psola`
continues; duration_warp nudges the per-phoneme timing so the lyric's
list-of-small-things rhythm lands.

## Praat treatment
Leans on **`duration_warp.praat`** from `nadir-praat-scripts` for the
per-phoneme timing of the list lines (bars 14–18), with
**`psola_retarget.praat`** for the pitch flattening. Pitch extraction
autocorrelation (`ac`), floor 60 Hz, ceiling 900 Hz, very-accurate on.
Target median 180 Hz. Range factor 0.70. Formant shift ratio 0.93 —
slightly darker, matching the older-paper feel. Duration factor 1.03 —
a small overall stretch.

## csdr graph shape
`band_limit` preset at -25 dB, 200–2200 Hz. Bed silenced for bars 1–2
(opening on the map alone) and for the final bar (closing on the key
we do not remember the lock for). No reverb.

## G2P / pronunciation hints
- "creases" as /'k r i s I z/ — voiced final /z/.
- "disagree" as /d I s @ 'g r i/ — stress on 'g r i'.
- "appropriate" as adjective /@ 'p r oU p r i @ t/ — final /t/, not /eI t/
  (the verb form).
- "thimble" as /'T I m b l=/ — match track 13 exactly; this is a quoted
  object.

## openSMILE gate
Voicing fraction in [0.30, 0.50]. Pitch-error RMS 2 cents ceiling.
Loudness -18 LUFS integrated.
