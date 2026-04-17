# 04 Kitchen Scale — notes

## Why this slot
A ritual object. We weigh flour every week. The phrygian mode's flat
second carries the minor-second half-step intimacy that matches the
small, careful weighing gesture. After two exterior objects (window,
glove) and one public object (station clock), we come indoors.

## Form and motif deployment
Through-composed, 18 bars in E phrygian at 58 bpm, 4/4. The lyric divides
into three mini-verses of roughly 5 lines each; the PSOLA treatment
tightens range slightly with each verse (0.75 → 0.72 → 0.68 effectively,
applied by scripting three passes through the same `psola_retarget.praat`
call chain on verse regions). `m.whisper_psola` continues.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts`. Pitch
extraction autocorrelation (`ac`), floor 70 Hz, ceiling 1000 Hz,
very-accurate on. Target median 205 Hz. Range factor 0.72. Formant shift
ratio 1.04 — a small upward shift that brightens the vowels without
un-seating them. We want the voice just slightly lighter than in the
bench-outdoor scene of track 02.

## csdr graph shape
`band_limit` preset at -24 dB, 250–2400 Hz. Bed gated off bars 1–3 and
16–18 so the first and last thoughts sit on silence. No reverb. The
kitchen is small; the voice is close; we do not add air.

## G2P / pronunciation hints
- "brass" as /b r & s/ — clean final /s/, do not voice into /z/.
- "needle" as /'n i d l=/ — syllabic /l=/, do not insert a schwa.
- "confession" as /k @ n 'f E S @ n/ — stress on 'f E S'.
- "weighed" and "weighing" — match the /w eI/ onset across both lines
  so the reprise is audible.

## openSMILE gate
Voicing fraction in [0.32, 0.50]. Pitch-error RMS 2 cents ceiling.
Loudness -18 LUFS integrated.
