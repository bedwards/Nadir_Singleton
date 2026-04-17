# DEV_WORKFLOW.md

## Branch & PR loop

1. **Issue first.** `gh issue create` with concrete acceptance criteria. Label: `area/vox|dsp|feat|praat|vad|compose|infra|album`.
2. **Branch.** `feat/<issue#>-short-slug` off `main`.
3. **Background worker.** Orchestrator spawns a subagent with the issue link. Worker opens a draft PR early.
4. **Review.** Gemini Code Assist GitHub App runs automatically on push. Wait for review summary.
5. **Capture reviews.** Every non-trivial Gemini comment → new GH issue with label `from-review`. Do NOT let threads linger in the PR.
6. **Merge.** Once CI green and review issues filed, squash-merge. Delete branch.
7. **Version bump + tag.** Bump Cargo workspace + Python pkg version. `git tag vX.Y.Z -m "..."`. `git push --tags`.
8. **No open PRs.** End of day: every PR is merged or closed.

## Versioning rules

- `v0.x.y` pre-1.0.
- Patch (`x.y.Z`): docs, research notes, album liner edits, bugfix.
- Minor (`x.Y.0`): new CLI subcmd, new crate, album ships.
- Major (`X.0.0`): pipeline break, CLI surface change.

## Commit style

Conventional commits. Short imperative subject. Body explains *why*.

```
feat(vox): pho emitter for MBROLA us1

Lyric→phoneme pipeline emits SAMPA with per-phoneme duration
and 4-point pitch contour. Needed for melody on album 01.
Closes #12.
```

## CI (`.github/workflows/`)

- `rust.yml` — `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.
- `python.yml` — `uv run ruff check`, `uv run pytest`.
- `render-smoke.yml` — renders a 4-second demo song on each PR, uploads artifact.
- `tools.yml` — verifies praat/opensmile/mbrola/silero/csdr presence, fails fast otherwise.

## Tools lockdown

`scripts/bootstrap.sh` installs the exact versions. Pins recorded in `tools/VERSIONS.toml`.

## Gemini review capture

`scripts/capture_gemini.sh <pr-number>`:
- `gh pr view <n> --comments --json comments,reviewThreads`
- Extracts each distinct finding (body + file:line)
- Creates issues titled `review: <first line>` with label `from-review`
- Replies in the PR thread with the linked issue so reviewer knows we tracked it

## Agent coordination

Orchestrator:
- Keeps TaskList of in-flight issues.
- Dispatches background Agent per issue with a self-contained brief.
- Monitors PR status via `gh pr list --json number,state,reviewDecision`.
- On completion → captures reviews, merges, bumps version, tags.

Background workers:
- Receive: issue #, acceptance criteria, file list to touch, constraints.
- Must not spawn further agents unless brief permits.
- Must open PR, not merge.

## Secrets

None in repo. `.envrc.example` shows required env (none for now).

## Git hygiene

- `.gitignore` covers: `target/`, `__pycache__/`, `.venv/`, `*.wav` under build dirs, `tools/build/`.
- Large WAV renders kept but LFS-gated once total > 1 GB (`git lfs track "albums/**/*.wav"`).
