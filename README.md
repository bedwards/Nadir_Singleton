# Nadir_Singleton

Experimental music produced exclusively with five open-source primitives:
**openSMILE · Praat · MBROLA · Silero-VAD · csdr**.

Nadir_Singleton is the AI composer/producer persona and the production system it runs on. Vocals are diphone-synthesised and pitch-corrected to key. DSP is hand-piped through csdr. Nothing leaves the constraint.

## Output

Twenty albums. 14–24 songs each. Narrative arcs within and across the corpus.
Current status: `albums/` (see each album's `MANIFEST.toml`).

## Quick start

```bash
# prerequisites: rustup, uv, homebrew (macOS)
./scripts/bootstrap.sh           # installs praat, builds mbrola/csdr, fetches opensmile, syncs uv/cargo
cargo run -p nadir -- --help     # or: cargo install --path crates/nadir && nadir --help
nadir song render --album 01_horizon_salts --track 03 --out out.wav
```

## Docs

- [`CLAUDE.md`](CLAUDE.md) — rules for agents working in this repo
- [`SPEC.md`](SPEC.md) — system spec
- [`DEV_WORKFLOW.md`](DEV_WORKFLOW.md) — ship process
- [`SONG_PRODUCTION.md`](SONG_PRODUCTION.md) — creative pipeline
- [`research/`](research/) — deep-dive notes on each core tool

## Constraint

If a feature needs a tool not in the five primitives, either express it via those tools, write custom Rust/Python glue, or cut the feature.

## License

MIT for code. Audio output CC-BY-SA 4.0.
