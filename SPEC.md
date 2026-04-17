# SPEC.md — Nadir_Singleton system

## Constraint set

Exactly five external tools may touch audio:

| Tool | Role | Interface |
|------|------|-----------|
| openSMILE | feature extraction (pitch, formants, spectral, eGeMAPS, ComParE) | CLI `SMILExtract` + config files |
| Praat | pitch/formant/duration manipulation, PSOLA resynthesis, TextGrid analysis | `praat --run script.praat` |
| MBROLA | diphone-concatenative vocal synthesis from phoneme+duration+pitch sequences | stdin `.pho` → stdout PCM |
| Silero-VAD | voice activity detection, speech segmentation | Python/ONNX |
| csdr | streaming DSP pipe blocks (FIR, FFT, mixer, resamplers, AM/FM mod-demod, shift, agc) | Unix pipeline of binaries |

Any other audio process = custom code atop these primitives, OR not allowed.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  nadir (Rust CLI, clap)                                 │
│  ├─ album   manage album manifests                      │
│  ├─ song    render / analyze / iterate                  │
│  ├─ vox     MBROLA-driven vocal synthesis               │
│  ├─ pitch   Praat PSOLA pitch/formant ops               │
│  ├─ vad     Silero voice activity detection / split     │
│  ├─ dsp     csdr pipeline builder + runner              │
│  ├─ feat    openSMILE feature extraction                │
│  ├─ corpus  cross-album query, narrative linking        │
│  └─ research  open research notes                       │
└─────────────────────────────────────────────────────────┘
                   │
        ┌──────────┼──────────┬──────────┐
        ▼          ▼          ▼          ▼
     nadir-       nadir-    nadir-     nadir-
     vox (Rs)     dsp (Rs)  feat (Rs)  compose (Rs)
        │          │          │          │
        ▼          ▼          ▼          ▼
   mbrola +    csdr       opensmile   praat
   pho-gen    binaries    SMILExtract  scripts
        │                               │
        └───── python (uv) glue ────────┘
                  │
              silero-vad
```

### Crates (Rust workspace at `crates/`)

- `nadir` — CLI entrypoint, subcommand dispatch, `--help` tree.
- `nadir-core` — shared types: `Score`, `Pho`, `Pitch`, `Clip`, `Stem`, `Track`, `Album`.
- `nadir-vox` — vocal synthesis: lyrics → phoneme+duration+F0 → MBROLA → raw PCM.
- `nadir-dsp` — csdr pipeline DSL → spawns `csdr` binary chain via pipes.
- `nadir-feat` — openSMILE wrapper, parses ARFF/CSV output, returns feature matrices.
- `nadir-praat` — generates Praat scripts, runs them, parses TextGrid / Pitch / Table output.
- `nadir-compose` — score model: scales, keys, meter, lyric→phoneme, melodic-motif engine.
- `nadir-render` — top-level render graph: mixes stems from the four tools through csdr bus.

### Python (uv workspaces at `python/`)

- `nadir-vad/` — Silero-VAD batch runner + segment export.
- `nadir-praat-scripts/` — library of `.praat` scripts (PSOLA, manipulation, formant shift).
- `nadir-lyric-g2p/` — grapheme-to-phoneme for MBROLA voice (US English `us1`/`us3`, etc.).
- `nadir-notebook/` — optional exploratory analysis, non-production.

## Data model (core types)

```rust
struct Album { id: String, title: String, slug: String, narrative: String, tracks: Vec<TrackRef> }
struct Track { n: u8, title: String, key: Key, scale: Scale, bpm: f32, bars: u32, meter: (u8,u8), parts: Vec<Part> }
enum Part { Vox(VoxPart), Bed(DspPart), Texture(CsdrPart), Sample(PraatPart) }
struct VoxPart { lyric: String, phonemes: Vec<Pho>, rhythm: Vec<Beat>, melody: Vec<Note>, mbrola_voice: String }
struct Pho { sampa: String, dur_ms: u32, pitch_points: Vec<(u32, f32)> } // (pct, hz)
```

## CLI surface (v0.1)

```
nadir --help
nadir album new <slug>
nadir album list
nadir song new --album <slug> --n <NN> --title ...
nadir song render --album <slug> --track <N> [--out path.wav]
nadir vox synth --pho file.pho --voice us1 -- [mbrola passthrough args]
nadir vox from-lyrics --text "…" --voice us1 --key C --scale minor --bpm 96
nadir pitch psola --in a.wav --target-f0 220 -- [praat args]
nadir vad split --in a.wav --out segs/
nadir dsp run --graph graph.toml -- [csdr passthrough]
nadir feat extract --config eGeMAPSv02 --in a.wav --out a.csv
nadir corpus narrative
nadir research open <tool>
```

Every level exposes `--help`. `--` forwards all remaining args verbatim to the wrapped tool.

## Output contract

- 48 kHz, 24-bit PCM WAV stems under `albums/<slug>/<NN>_track/stems/*.wav`
- Mixdown at `albums/<slug>/<NN>_track/mix.wav`
- Album liner at `albums/<slug>/LINER.md` (story)
- Track manifest at `albums/<slug>/<NN>_track/manifest.toml`

## Reproducibility

Every render records: tool versions, exact CLI invocations, RNG seeds, input hashes. Captured in `albums/<slug>/<NN>_track/render.lock.toml`.

## Non-goals

- Real-time performance
- Third-party plugin hosting (VST/AU/LV2)
- MIDI I/O to external hardware
- Training ML models
