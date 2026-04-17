# SONG_PRODUCTION.md — creative pipeline

## Principle

A Nadir_Singleton song is a *graph of tool outputs summed on a csdr bus*. Vocals lead; rhythm and texture follow. Every song must exercise all five core tools audibly or structurally.

## Song blueprint

```
score.toml (key, scale, bpm, meter, form) ─┐
lyric.txt ─┐                                ├─► compose ─► plan.toml
           ▼                                │
         g2p (python) ─► phonemes ──────────┤
                                            ▼
                                 ┌──────────────────────┐
                                 │  render graph (TOML) │
                                 └──────────────────────┘
                                            │
     ┌───────────────┬──────────────┬───────┴───────┬──────────────┐
     ▼               ▼              ▼               ▼              ▼
  MBROLA vox    Praat PSOLA     csdr texture    Praat-shaped    openSMILE
  (raw PCM) ──► (tuned to key) ─► beds          found-sample    feature
                                                 impulses       audit
     │               │              │               │              │
     └───────────────┴──────────────┴───────────────┘              │
                              │                                    │
                        Silero-VAD                                 │
                        slices & onsets                            │
                              │                                    │
                              ▼                                    │
                     csdr mixer / shift / FIR ───► stems ───► mix ◄┘
```

### Stage details

1. **Score.** `score.toml`: key (e.g. `A`), scale (`minor`, `dorian`, custom `[0,2,3,5,7,8,10]`), bpm, meter, form (`AABA`, `verse/chorus/bridge`, `through-composed`).
2. **Lyric.** `lyric.txt`. Tin-Pan-Alley-tight or abstract. Syllabified.
3. **G2P.** `nadir vox from-lyrics` uses `nadir-lyric-g2p` → SAMPA phoneme sequence matched to MBROLA voice inventory (`us1`, `us3`, `en1`, `fr1`, etc.).
4. **Melody.** `nadir-compose` assigns pitches from the scale to syllables, honouring contour rules (stepwise bias, tessitura, phrase arcs). Rhythm assigns durations on the bpm grid.
5. **Vox synth.** Phonemes + durations + F0 contour → `.pho` file → MBROLA → 16 kHz PCM (upsampled via csdr `convert_s16_f` → `fir_interpolate` → 48 kHz).
6. **Pitch correction.** Praat PSOLA snaps F0 to scale tones, adds vibrato, micro-tuning, optional formant shift for vocal character.
7. **Texture beds.** csdr pipelines: shaped noise through FIRs, ring-modulated with carriers derived from MBROLA F0, FFT-domain spectral brushes. Each bed is a Unix pipe of csdr blocks.
8. **Rhythm.** Silero-VAD on the raw vocal → syllable onsets → timing grid → drives texture gates (csdr `dsb_fc` amplitude envelopes) and sample-impulse hits (Praat-generated Dirac/formant clicks).
9. **Feature audit.** openSMILE eGeMAPSv02 on each stem + the mix. Thresholds: loudness peak, spectral-flux density, voicing fraction. Out-of-bound stems → automatic regen with adjusted params. Features stored at `render.features.csv`.
10. **Mixdown.** csdr mixer sums stems with per-stem gain and pan. Final limiter = csdr `fastagc_ff` + clip guard.

## Vocal key/scale tuning (core innovation)

Pipeline `nadir vox tune`:

1. MBROLA emits raw speech with a neutral F0 contour per syllable.
2. openSMILE extracts realized F0 track at 10 ms hop.
3. `nadir-compose` computes target F0: snap realized pitch centroid per syllable to nearest scale degree in the chosen key, then apply melodic plan.
4. Praat script `psola_retarget.praat` does PSOLA resynthesis with the target F0 contour.
5. openSMILE re-audits pitch error. Iterate until RMS error < 2 cents or 3 passes.

Result: computer vocals that enunciate lyrics in tune to a key and scale.

## Album narrative

- 20 albums total. Target corpus: ~380 tracks.
- Each album has a `LINER.md` with story arc, character(s), motifs.
- Cross-album motifs tracked in `albums/CORPUS.md`: recurring phoneme clusters, F0 motifs, DSP signatures.
- Track order matters — `MANIFEST.toml` is canonical.

### Narrative arc (first draft)

```
01  Horizon Salts      — awakening, first utterance
02  Tin Pan Fathom     — old forms, new throat (TPA homage)
03  The Spectral Fair  — carnival of feature vectors
04  VAD Epistles       — epistolary, each track a letter
05  Phoneme Monastery  — ritual, repetition, vowel drones
06  csdr Weather       — field recordings synthesised from DSP alone
07  Praat Lieder       — art-song cycle, single voice + minimal bed
08  MBROLA Cabaret     — jazzy, ragtime re-imaginings
09  Formant Gardens    — ambient / botanical
10  The Cent Maze      — micro-tonal scales, quarter-tones and less
11  Voicing Fraction   — pop structures, hooks, refrains
12  Diphone Drift      — slowly evolving, tape-like
13  eGeMAPS Tarot      — 22 tracks, one per feature-family archetype
14  Silero Rooms       — onset-driven percussive
15  FIR Psalms         — hymns through csdr FIR shaping
16  Pitch Pilgrimage   — modal journey, scale per track
17  Plosive Letters    — consonant-forward rhythmic lyric
18  Dorian Weather     — modal pop
19  Singleton Suite    — long-form, 24 connected tracks
20  Nadir              — coda, de-resolution, silence
```

## Quality gates

A track ships only when:
- Every five core tools appear in `render.lock.toml` invocations.
- openSMILE eGeMAPS pitch-error RMS ≤ 2 cents on vocal stem.
- Mix peak ≤ -1 dBTP, integrated loudness ≈ -14 LUFS (measured via csdr + custom meter).
- A human-readable `NOTES.md` in the track folder explains choices.

## Iteration loop

Orchestrator owns a rendering queue. Workers pull a track, render, report features, push artifacts. Failures re-enter queue with adjusted params. Budget per track: 8 passes max.
