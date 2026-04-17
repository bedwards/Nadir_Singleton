# TIME_LOG — Nadir_Singleton

Per-task estimation vs actual. Discipline: before starting, estimate; after, measure; note delta.

All deltas derived with `python -c`. Timestamps via `date`.

Format per entry:
```
YYYY-MM-DD HH:MM   ref: PR#N / issue#N / task
  estimate:  X min   actual: Y min   delta: +Z
  user-observable: one-line description of what they see/hear/run
  notes: any rules-of-thumb material
```

---

## 2026-04-17 (session 1 — retroactive, UTC timestamps from PR merges)

Session launch: user asked for autonomous feature sprint. Ran 11 PRs in ~25 minutes of wall time. Timestamps are PR-merge UTC (CDT = UTC-5, merges were roughly 09:20–09:45 CDT).

```
09:20:50   PR#43 feat(compose): BPM-driven syllable timing   already-written, commit+test+PR+merge only
  estimate: n/a (pre-tracking)   actual: n/a   delta: n/a
  user-observable: stressed syllables get full beat, unstressed half beat
```

```
09:22:04   PR#44 feat(song): stems dir
  actual: 1.2 min    user-observable: raw_vox.wav & tuned_vox.wav saved per track for A/B listening
  rule: single-dispatch-fn edit with obvious path fields → ~1–2 min
```

```
09:22:57   PR#45 feat(cli): nadir play
  actual: 0.9 min    user-observable: `nadir play <wav>` spawns afplay/aplay
  rule: adding a clap subcommand that shells out to one binary → <1 min
```

```
09:28:08   PR#46 feat(render): shaped-noise bed + mix
  actual: 5.2 min    user-observable: songs have shaped-noise bed under vocal; first mixed track
  rule: introducing a new multi-module API (read WAV, run csdr generator, mix, write WAV) → 4–6 min
```

```
09:30:49   PR#47 feat(render): VAD-driven pulse track
  actual: 2.7 min    user-observable: percussive pulses land on syllable onsets
  rule: one-new-function that reuses existing helpers (pulse gen + band_limit_via_csdr) → 2–3 min
```

```
09:33:26   PR#48 feat(audit): openSMILE pitch audit
  actual: 2.6 min    user-observable: stems/audit.json with rms_cents
  rule: adding a helper that wraps an existing tool invocation + CSV parsing + JSON dump → 2–3 min
```

```
09:36:50   PR#55 feat(render): tonal triad drone bed
  actual: 3.4 min    user-observable: harmonic bed instead of noise; tonal_drone_triad preset
  rule: adding a new preset kind + enum refactor → 3–4 min
```

```
09:38:36   PR#56 feat(song): CLI overrides + silence openSMILE stderr
  actual: 1.8 min    user-observable: `song render --bpm 72 --bed-preset ...` works; no more openSMILE spam
  rule: bundling two small quality-of-life changes in one PR → ~2 min
```

```
09:41:20   PR#57 feat(compose): phrase-shaped melodic contour
  actual: 2.7 min    user-observable: melodies arc per phrase instead of random-walk
  rule: new compose fn + cli wiring + test → 3 min
```

```
09:43:31   PR#58 fix(audit): octave-fold realized F0
  actual: 2.2 min    user-observable: audit now reports realistic ~19¢ instead of 47¢
  rule: diagnose-with-python-one-liner + helper fn + test + tweak cli → ~2 min
```

```
09:45:50   PR#59 feat(album): album render + album play
  actual: 2.3 min    user-observable: `nadir album render <slug>` renders every track; `play` listens through
  rule: self-spawn subcommand with filesystem walk + sort → 2–3 min
```

### Session 1 aggregate

```
python -c "deltas=[1.2,0.9,5.2,2.7,2.6,3.4,1.8,2.7,2.2,2.3]; import statistics; print('n',len(deltas),'sum',sum(deltas),'mean',round(statistics.mean(deltas),2),'median',statistics.median(deltas),'min',min(deltas),'max',max(deltas))"
# n 10 sum 25.0 mean 2.5 median 2.45 min 0.9 max 5.2
```

10 user-observable features/fixes in 25 minutes, mean 2.5 min/PR, median 2.45. Range 0.9 (trivial CLI add) to 5.2 (new multi-module bed + mix infrastructure).

---

## rules of thumb (v0.1)

Derived from session 1 only — treat as seed estimates, refine each session.

| pattern | expected minutes |
|---|---|
| add clap subcommand that shells to one binary | 1 |
| stems dir / filename reshuffle in one dispatch fn | 1–2 |
| new helper fn + test + one cli call site | 2–3 |
| new preset + enum refactor + two call sites | 3–4 |
| first-time multi-crate feature (API + cli wiring) | 4–6 |
| bug diagnosis w/ python analysis + single fix | 2–3 |
| quality-of-life bundle (flag + silence noise) | 2 |

Ceilings to watch for: `cargo build --release` for the whole workspace after a lib change is ~10–20 s; `song render` end-to-end is ~3–5 s per track; full album 01 render is ~60–80 s for 14 tracks. Keep these out of per-PR estimates — they're fixed-cost backdrop.
