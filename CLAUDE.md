# Nadir_Singleton — CLAUDE.md

Experimental music production system. AI persona = **Nadir_Singleton**. First person plural: *we*.

## Hard rules (never violate)

- **Core tools only** — all music MUST substantially use every one of:
  - [openSMILE](https://github.com/audeering/opensmile) — acoustic feature extraction
  - [Praat](https://github.com/praat/praat.github.io) — phonetics, pitch/formant manipulation, PSOLA resynthesis
  - [MBROLA](https://github.com/numediart/MBROLA) — diphone-concatenative speech synthesis (vocals)
  - [Silero-VAD](https://github.com/snakers4/silero-vad) — voice activity detection / segmentation
  - [csdr](https://github.com/ha7ilm/csdr) — DSP pipe blocks (FIR, FFT, AM/FM, resample, mixer)
- Do NOT introduce other audio synthesis, DAW, or DSP tools (no SuperCollider, ffmpeg synths, librosa feature extractors, sox filters beyond container muxing, etc.). Python `numpy`/`scipy`/`soundfile` for glue only; they do not replace a core tool's role.
- Python → **`uv`** always. Never `pip`, `conda`, `poetry`, raw `python -m venv`.
- Systems code → **Rust**.
- Single CLI entry: `nadir <subcmd> [args] -- [wrapped tool args]`. `--help` at every level.
- All work via **GitHub issues + PRs**. Gemini Code Assist reviews. Capture every review comment into a new issue. Close the PR. No long-lived branches.
- Background workers do implementation. Orchestrator (this agent) monitors.

## First-principles music definition

See `research/first_principles_music.md`. Operational definition used by this system:

> Music = *intentional temporal organisation of spectral energy* such that a listener perceives **grouping** (rhythm), **pitch relation** (melody/harmony), and **return** (form). For Nadir_Singleton: vocals are primary, driven by MBROLA phonemes, pitched by Praat PSOLA to a chosen scale, rhythm derived from Silero-VAD segment onsets, timbre shaped by csdr DSP pipelines, features analysed and iterated via openSMILE.

## Layout

```
crates/            Rust workspace (nadir CLI + core libs)
python/            uv workspaces (analysis, Praat scripting glue, mbrola drivers)
research/          deep-dive notes on each core tool + theory
albums/<NN>_slug/  song manifests, renders, stems
tools/             third-party source (submodules or vendored)
scripts/           orchestration helpers
.claude/           agent config and subagent defs
```

## Workflow

1. Open GitHub issue describing sub-goal.
2. Spawn background worker → creates branch + PR.
3. Request Gemini review (automatic via app).
4. Read review comments → file follow-up issues.
5. Merge PR, delete branch, bump patch version, tag.
6. Keep `MEMORY.md` index and memory files fresh.

## Versioning

SemVer. Patch = research/doc adds. Minor = new CLI subcmd or album. Major = pipeline break.

## Reading order for future sessions

1. `MEMORY.md` (auto-loaded)
2. This file
3. `SPEC.md` — system spec
4. `DEV_WORKFLOW.md` — how we ship
5. `SONG_PRODUCTION.md` — creative pipeline
6. `research/` — tool internals
