# Nadir_Singleton — 100-Video YouTube Release Plan

This document captures the production, mastering, video, and upload-scheduling
plan for a 100-video YouTube release series. Treat this as the single source
of truth. Update in place when decisions change.

## Goal

Ship **100 videos at approximately 11.5 minutes each** (1150 minutes of audio
compiled into 100 long-form YouTube uploads, no Shorts, no 1–4-hour
background-music films).

Math (verify with `python3 -c`):

```
python3 -c "print(100 * 11.5, 'min total'); print(1150/3, 'songs at 3 min each')"
# 1150.0 min total
# 383.33... songs at 3 min each
```

**Target: ~383 songs at ~3 min each, compiled into 100 × 11.5-minute videos.**
That's 20 albums × ~19 tracks/album, which matches the album catalogue already
planned in GitHub issues #11–#29.

## Hard constraints (from the user, documented so we don't relitigate)

- **11.5-minute target duration.** Not 3 minutes. Not 1–4 hours. Even if
  generic YouTube research says 3 minutes is optimal for music, ignore it —
  this music is not 3-minute-single-friendly. Even if research says long-form
  "background" video should be 1–4 hours, ignore it — that's a different
  format with different viewer expectations.
- **No Shorts. Ever.**
- **Loudness chain:** each mixed track hits **-18 dB integrated LUFS**;
  the **master** hits **-9 dB integrated LUFS**; hand that to YouTube, which
  will attenuate to roughly -14 LUFS as part of its normalization. We are
  deliberately over-shooting YouTube's target so YouTube's downrange is the
  limiter, not our mastering chain.
- **100% automated.** No hand-editing frames or levels.
- **All OSS.** Core audio: openSMILE, Praat, MBROLA, Silero-VAD, csdr (the
  five-tool constraint from `CLAUDE.md`). Video: see "Visual pipeline" below.
- Existing tools on the box: Bevy engine (`~/vibe/bevyn`), ImageMagick,
  ffmpeg, Blender, zimage. These are all fair game.

## Content inventory (as of 2026-04-18)

- **Current rendered audio:** album 01 "Horizon Salts", 14 tracks at ~20 s
  each (~5 minutes total). Far short of the 1150 min target.
- **Planned but unbuilt:** 19 more albums (issues #11–#29 track each). Each
  album is planned at 14–22 tracks.
- **Per-track duration today:** 20–25 seconds. Needs to grow to ~3 minutes.

### Path to 1150 minutes

Pick one or a combination:

1. **Longer lyrics per track** — current tracks are 20–24 short lines;
   expanding to verse+chorus+bridge structure with full choruses repeated
   gets tracks to 2–3 min easily.
2. **More tracks per album** — budget ~19 tracks per album, 20 albums → 380
   songs, close enough to 383.
3. **Section repeats at render time** — `nadir song render` could repeat the
   whole composed phrase stream N times (e.g. A-B-A-B-bridge-A structure)
   to stretch short lyrics into long songs.

Recommended: generate long lyrics (option 1) for future albums; use section
repeats (option 3) for existing short albums so album 01 remains re-usable.

## Compilation rule

`nadir compile --target-minutes 11.5 --out videoNN.wav`:

1. Walk all rendered tracks in a fixed album → track order.
2. Greedy-pack: append tracks until total duration ≥ 11.5 min.
3. **Decide last song by closer-to-target rule:** if including the candidate
   makes |total − 11.5 min| smaller than excluding it, include it; otherwise
   don't.
4. Start a new compilation at the next unused track. Don't worry about
   "clean" album starts or ends between compilations.
5. Short cross-fade (1–2 s) between songs at compile time.

## Mastering chain (-18 LUFS mix → -9 LUFS master)

Per-stem normalization already hits -18 dBFS integrated (RMS proxy) in
`nadir song render`. The compilation step masters the chained mix to -9
integrated LUFS for YouTube.

```bash
# Two-pass ffmpeg loudnorm — exact integrated LUFS control
ffmpeg -i compile.wav \
  -af "loudnorm=I=-9:TP=-1:LRA=7:measured_I=${MEASURED_I}:measured_LRA=${MEASURED_LRA}:measured_TP=${MEASURED_TP}:measured_thresh=${MEASURED_THRESH}:offset=${OFFSET}:linear=true:print_format=summary" \
  -c:a pcm_s24le master.wav
```

Pass 1 reads `loudnorm=I=-9:TP=-1:LRA=7:print_format=json` output to get
the `measured_*` values, then pass 2 uses them with `linear=true` so the
gain correction is a single scale factor (no dynamics distortion).

## Visual pipeline (research findings, 2026-04-18)

Five highly-regarded OSS GitHub projects **not intended for music video**
that can be scripted headlessly on macOS:

1. **Cinder** — https://github.com/cinder/Cinder. C++ creative-coding
   framework (BSD). Long GPU shader beds, particle fields, long-exposure
   trails. Offscreen FBO → PNG sequence from CLI.
2. **Mitsuba 3** — https://github.com/mitsuba-renderer/mitsuba3. Physically
   based renderer (BSD-3). Python-scriptable volumetric media, subsurface
   scattering, caustics — matches the "salt / horizon / mirror" motif of
   album 01.
3. **ParaView / VTK** — https://github.com/Kitware/ParaView. Scientific
   visualization (BSD). `pvbatch` headless script reads CSV of openSMILE
   features per frame, updates filter, saves screenshot.
4. **Mandelbulber v2** — https://github.com/buddhi1980/mandelbulber2.
   3D fractal renderer (GPL-3). `mandelbulber2 --nogui --keyframe-anim
   settings.fract`.
5. **Pixray** — https://github.com/pixray/pixray. CLIP-guided
   image/video generator (MIT). Driven directly from MBROLA phoneme
   strings or lyric lines — unique lyric→image binding. MPS backend on
   Apple Silicon.

Plus the already-installed local tools (Bevy, ImageMagick, ffmpeg,
Blender, zimage) for frame post-processing, muxing, and procedural 3D.

### Per-video visual strategy

- Pick the tool per album (not per track) so each album has a visual
  identity. Example assignments:
  - Album 01 "Horizon Salts" → Mitsuba 3 participating media
  - Album 02 "Tin Pan Fathom" → ParaView flow fields
  - Album 03 "The Spectral Fair" → Bevy + shaders
  - Album 04 "VAD Epistles" → Mandelbulber keyframed zoom
  - Album 05 "Phoneme Monastery" → Pixray prompted by phoneme stream
  - etc. Assignments TBD for remaining albums.
- All visuals render at 3840×2160 (4K) 60fps so ffmpeg encode matches
  folk-sequence's existing settings.

## YouTube upload schedule

Based on April 2026 research (Buffer 1.8M-video study, vidIQ 5.08M-channel
study, YouTube Help, Backlinko 1.3M-result study). Generic SEO-blog advice
and Reddit opinions disregarded per constraint.

### Slots

- **Primary: Sunday 14:00 UTC (10:00 ET / 07:00 PT).** Highest observed
  engagement factor (0.95) for long-form music content in Buffer's 2026
  analysis.
- **Secondary: Tuesday 14:00 UTC.**
- **Tertiary: Friday 14:00 UTC.**

Hits US East morning + EU early evening; both fit ambient/meditative
listening contexts.

### Cadence

**3 uploads per week** (Sun + Tue + Fri) = ~13/month. vidIQ's 5M-channel
study shows the growth curve steepens dramatically at 12+ uploads/month;
going higher (daily) risks YouTube's recommendation system flagging the
channel as spammy and doesn't produce proportional signal on an
11.5-minute format.

### Release schedule span

100 videos ÷ 3/week ≈ **33.3 weeks ≈ 7.5 months** of drip-releases.

```
python3 -c "print(f'{100/3:.2f} weeks = {100/3/4:.2f} months')"
# 33.33 weeks = 8.33 months
```

### Batch vs drip

**Drip, never batch.** Every credible 2025–2026 source says bulk-drops
suppress distribution: unclicked notifications and skipped impressions
get logged as negative signal. Schedule all 100 as future-dated uploads
in YouTube Studio, keep a ~4-week buffer so a bad week never breaks
consistency.

### Success metrics

- **Month 1 (~13 videos):** Average View Duration ≥ 4:00 (~35% of 11.5
  min). Ignore sub/view ratio and CTR this early — too noisy.
- **Month 3 (~40 videos):** Browse/Suggested impressions > 40% of total,
  returning-viewers % trending up, subscriber growth faster than month
  1. If flat, shift one slot to Saturday and re-test in month 4–6.

## Upload integration (folk-sequence)

`~/vibe/folk-sequence` has an existing YouTube upload pipeline via
Google Data API v3. Reuse its OAuth + scheduling machinery. **Key
deviation:** folk-sequence encodes audio with `loudnorm=I=-14:TP=-1:LRA=11`
which targets YouTube's own integrated loudness. We intentionally
target **-9** in our mastering step so YouTube's normalization does the
final attenuation — the chain produces more headroom and a louder
perceived result than meeting YouTube halfway.

Adaptation plan: add a `folkseq transcode --loudness -9` (or new
`--loudness auto-nadir`) flag OR run our own ffmpeg encode upstream
and hand folk-sequence a pre-encoded .mp4 with no further loudnorm
applied.

## Ship order

1. `nadir compile` CLI — chain-and-crossfade songs into ~11.5-min WAVs.
2. ffmpeg loudnorm wrapper — two-pass -9 LUFS master.
3. Song generation at 3-minute scale (issue for future PRs).
4. Visual pipeline scaffolding — one album chosen, one tool wired.
5. YouTube upload hook via folk-sequence (loudness-patched).
6. Full 100-video schedule generated in the format
   folk-sequence/schedule.json expects.

## Decision log

- **2026-04-18** Chose -9 LUFS master because user wants YouTube's
  -14 attenuation to do the compression floor. Rejected matching -14
  upstream.
- **2026-04-18** Chose 3/week Sun+Tue+Fri 14:00 UTC schedule over
  daily or weekly alternatives based on vidIQ 12+/month threshold
  and Buffer Sunday-morning engagement peak.
- **2026-04-18** Chose 11.5-min target, explicitly overriding
  research-consensus 3-min-song and 1–4h-background formats. User's
  editorial call; documented here so future work doesn't question it.
- **2026-04-18** Chose to drip-release all 100 videos (never batch).
