# Praat — Research note (April 2026)

Operating manual for using Praat inside the Nadir_Singleton pipeline. Assumes no prior familiarity with phonetics software. Covers headless script invocation, PSOLA F0 retargeting, formant shifting, TextGrid timing, and KlattGrid synthesis. We treat Praat as a black-box DSP server: scripts in, WAV + Table out.

## What it is

Praat ("speech" in Dutch) is a free, open-source phonetics workbench developed by **Paul Boersma** and **David Weenink** at the Institute of Phonetic Sciences, University of Amsterdam. Development began in 1992; it is the canonical environment for acoustic phonetics and speech synthesis research.

The design centre is *doing phonetics by computer*: measuring F0, formants, intensity, duration; annotating segments; resynthesising manipulated audio. Every capability we need in music production (pitch correction, formant shifting, time-warping, source-filter synthesis) is present, framed in phonetics vocabulary.

**Latest release as of April 2026:** version **6.4.63**, released **2026-04-04**. Distributed as a single self-contained binary. Source under CC-BY-SA 4.0. GitHub docs mirror: https://github.com/praat/praat.github.io. Source: https://github.com/praat/praat.

Praat is *not* a DAW, *not* a plugin, and has no real-time streaming. All operations are offline, whole-file, deterministic — ideal for a reproducible pipeline.

## Installation

### macOS

```bash
brew install --cask praat
```

Installs `Praat.app` into `/Applications/`. CLI binary at `/Applications/Praat.app/Contents/MacOS/Praat`. Symlink:

```bash
ln -s /Applications/Praat.app/Contents/MacOS/Praat /usr/local/bin/praat
```

Apple Silicon builds are universal since 6.3.x.

### Linux

Debian/Ubuntu: `apt install praat`. For latest, download the static binary from https://www.fon.hum.uva.nl/praat/ (~20 MB, glibc + minimal X11 for GUI). For containers, use the `praat_barren` build (no GTK dependency).

### Windows

Installer or standalone ZIP from the same page. In CI use the barren build, invoke `Praat.exe --run`.

### Headless / script mode

Non-interactive CLI via `--run`. Full CLI contract in *Running scripts headlessly* below.

## Scripting: the Praat scripting language

Praat has its own scripting language. It looks superficially like a mix of BASIC and shell, but its distinguishing feature is the **object stack**: every menu command either consumes selected objects and produces new objects, or queries a selected object for a value.

### Variables and types

```praat
sampleRate = 44100           ; numeric
f0_median# = {100, 120, 140} ; numeric vector (note the # suffix)
name$ = "vocal_take_03"      ; string (note the $ suffix)
```

Sigil conventions: `#` = vector, `##` = matrix, `$` = string, no sigil = scalar numeric. This is the single most confusing syntactic quirk to keep in mind.

### Control flow

```praat
for i from 1 to 10
    appendInfoLine: "iteration ", i
endfor

if f0 > 200
    kind$ = "high"
elsif f0 > 120
    kind$ = "mid"
else
    kind$ = "low"
endif

while t < duration
    t = t + 0.01
endwhile
```

### The object stack

Praat maintains a global list of objects. Commands execute against *currently selected* objects. The canonical pattern:

```praat
sound = Read from file: "input.wav"
selectObject: sound
pitch = To Pitch: 0.0, 75, 600
selectObject: pitch
f0_mean = Get mean: 0, 0, "Hertz"
```

The return value of a creation command is a numeric **object ID**. Pass it to `selectObject:` (or combine with `plusObject:`, `minusObject:`) to control the selection. Use `removeObject:` to free it.

### Forms (interactive input)

```praat
form Retarget F0
    sentence Input_file input.wav
    positive Target_median_hz 220
    real Range_factor 1.0
endform
```

When a script with a `form` is run headlessly via `--run`, the form is skipped and CLI arguments fill the fields in order.

### Procedures

```praat
procedure retarget .input$ .median
    Read from file: .input$
    sound = selected("Sound")
    manipulation = noprogress To Manipulation: 0.01, 75, 600
    ; ... mutate pitch tier ...
endproc

@retarget: "in.wav", 220
```

Dot-prefixed names (`.input$`, `.median`) are local to the procedure. Without the dot, the variable is global.

### Quirks

- Colons after commands (`To Pitch: 0.0, 75, 600`) mandatory in modern call syntax.
- Command names match menu text (ellipses → colons in scripts).
- Decimal separator is always `.` in scripts; see *Known pitfalls* for I/O locale issues.
- `noprogress` prefix suppresses the progress dialog. Always use in headless scripts.

## Running scripts headlessly

```bash
praat [--no-pref-files] [--ansi] --run script.praat arg1 arg2 ...
```

Flags:

- `--run` — execute and exit. Info-window output → stdout; errors → stderr; non-zero exit on error.
- `--no-pref-files` — do not read/write `prefs5.ini` / `buttons5.ini`. **Always in CI.**
- `--ansi` — force plain ASCII output (Windows).

Args fill `form` fields in order (whitespace-separated, quote to group).

Python invocation (mirrored in our Rust integration):

```python
import subprocess
subprocess.run(
    ["praat", "--no-pref-files", "--run", "psola_retarget.praat",
     "in.wav", "out.wav", "220"],
    check=True, capture_output=True, text=True,
)
```

Non-zero exit = script error with message on stderr. stdout = whatever the script `appendInfoLine`d. Our convention: tab-delimited machine-readable output, never pretty-printed.

## Core objects

The object types you will use:

- **Sound** — mono/stereo PCM as doubles; arbitrary sample rate and start time.
- **Pitch** — time-aligned F0 contour with per-frame candidate structure.
- **PointProcess** — set of time points; for voiced speech, glottal pulse locations. Consumed by PSOLA.
- **Formant** — time-aligned track of N formants (typically 5) with bandwidths.
- **Intensity** — RMS loudness contour in dB.
- **Manipulation** — composite: Sound + PointProcess + PitchTier + DurationTier. Master object for PSOLA resynthesis.
- **PitchTier** — time-stamped F0 targets, interpolated.
- **DurationTier** — time-stamped local-speed factors (1.0 = unchanged, 2.0 = half speed).
- **TextGrid** — annotation: multiple IntervalTier or PointTier layers.
- **Table** — tab-delimited tabular data. Our export format for F0/formant tracks.
- **Spectrogram** — 2D magnitude spectrum over time.

## PSOLA resynthesis

**PSOLA** (Pitch-Synchronous Overlap-Add) is the primary resynthesis engine for pitch and duration modification in Praat. It works in the time domain, slicing the signal around each glottal pulse into Hann-windowed grains, then reassembling them at new pulse positions dictated by the target PitchTier and DurationTier.

### Object flow

```
Sound  --> [To Manipulation]  -->  Manipulation {Sound, PointProcess, PitchTier, DurationTier}
                                         |
                                         v
                          [Get resynthesis (overlap-add)]
                                         |
                                         v
                                     Sound (modified)
```

Steps inside `To Manipulation`:

1. **Pitch analysis** with given time step, floor, ceiling.
2. **Pulse detection**: glottal pulses computed from pitch and waveform peaks.
3. **PitchTier creation**: one target point per pulse.
4. **Empty DurationTier** initialised (identity duration).

You then mutate the PitchTier (and/or DurationTier) and ask for the overlap-add resynthesis.

### Working script: retarget F0 contour to a target median

```praat
# psola_retarget.praat
# Usage: praat --no-pref-files --run psola_retarget.praat in.wav out.wav 220 1.0

form PSOLA retarget
    sentence In_path
    sentence Out_path
    positive Target_median_hz 220
    positive Range_factor 1.0
endform

Read from file: in_path$
sound = selected("Sound")

selectObject: sound
manipulation = noprogress To Manipulation: 0.01, 75, 600

selectObject: manipulation
pitchTier = Extract pitch tier
selectObject: pitchTier

# Current median in Hz
old_median = Get quantile: 0, 0, 0.5, "Hertz"

# Scale all points: new = old * (target/old_median)
# Then apply range factor around target_median_hz.
numPoints = Get number of points
for i from 1 to numPoints
    t = Get time from index: i
    old_f0 = Get value at index: i
    scaled = old_f0 * (target_median_hz / old_median)
    final = target_median_hz + (scaled - target_median_hz) * range_factor
    Remove point: i
    Add point: t, final
endfor

# Put the mutated PitchTier back into the Manipulation
selectObject: manipulation
plusObject: pitchTier
Replace pitch tier

# Resynthesise
selectObject: manipulation
resynth = Get resynthesis (overlap-add)
selectObject: resynth
Save as WAV file: out_path$

# Cleanup
select all
Remove
```

This script is the canonical pattern: load, manipulate, `Get resynthesis (overlap-add)`, save, clean up.

### Caveats

- DurationTier is multiplicative: 2.0 = half speed. Neutral 1.0. Add both anchor points per segment.
- `Get resynthesis (LPC)` exists on some builds; stick to overlap-add for pitch/duration.

## Pitch detection

Three algorithms are exposed. All produce a Pitch object.

### Autocorrelation (AC) — `Sound: To Pitch (ac)...`

Boersma's 1993 method. The default and usually best choice for clean speech or vocal takes. Parameters:

- **Time step (s)** — frame hop. Default 0.0 → Praat picks `0.75 / pitch_floor`, giving 3-period analysis windows. For music, 0.005 s (5 ms) is a good fixed value.
- **Pitch floor (Hz)** — candidates below are discarded. Default 75 for adult speech; for bass vocals go to 50. Determines window length (3 periods).
- **Pitch ceiling (Hz)** — candidates above ignored. Default 600. For soprano vocals or falsetto, raise to 1000–1500. For instrumentals, set intentionally.
- **Max candidates per frame** — default 15.
- **Very accurate** — doubles window length. Slower, cleaner contour.
- **Silence threshold**, **voicing threshold**, **octave cost**, **octave-jump cost**, **voiced/unvoiced cost** — dynamic-programming path-selection weights.

### Cross-correlation (CC) — `Sound: To Pitch (cc)...`

Alternative that correlates the signal against itself with variable lag. Slightly more robust to additive noise but more prone to octave errors on periodic music signals. Same parameter set.

### Subharmonic summation (SHS) — `Sound: To Pitch (shs)...`

Hermes 1988. Computes F0 by summing harmonics in a log-compressed spectrum. Used primarily for low-quality recordings (telephone speech) where autocorrelation fails. Parameters include a harmonic compression factor and a maximum frequency component.

### Tradeoffs

| Method | Strength | Weakness |
|---|---|---|
| AC | Best accuracy on voice, good SNR handling | Slightly sensitive to formant-adjacent partials |
| CC | Robust to impulsive noise | Octave errors on harmonic-rich signals |
| SHS | Handles missing fundamentals | Less precise in time |

For singing voice, **AC with pitch floor 65, ceiling 1000, very-accurate on** is our working default.

## Formant manipulation

Formants are spectral resonances of the vocal tract. Shifting them alters perceived vocal size / gender / character while preserving F0 and phonetic identity (up to a point).

### Extraction: LPC methods

- **Burg** (`Sound: To Formant (burg)...`) — most common. Computes LPC via the Burg recursion. Parameters: time step (default 0.025 s window, 0.01 s hop), max formants (5 for adult male, 5.5 for female), formant ceiling (5000 Hz male, 5500 Hz female, 8000 Hz children), window length (0.025 s), pre-emphasis from 50 Hz.
- **Autocorrelation** (`To Formant (ac)...`) — less robust at edges but more stable in spectral analysis.
- **Robust** (`To Formant (robust)...`) — iterative reweighting, slower but resistant to outliers in glottal-closed intervals.

The Burg method internally resamples the signal to 2× the formant ceiling, applies pre-emphasis, runs LPC of order `2 × max_formants`, solves for poles, and filters out poles below 50 Hz or above ceiling−50 Hz.

### All-at-once transformation: `Change gender...`

The highest-level Praat command for voice-character work. It performs four operations in sequence:

1. Resample the signal by `formant_shift_ratio` (ratio 1.1 → shift formants up ~10%).
2. Detect pitch.
3. Scale F0: `new = old × (target_median / old_median)`, then `final = target_median + (new - target_median) × range_factor`.
4. Apply PSOLA with optional `duration_factor`.

Signature:

```praat
Change gender: pitch_floor, pitch_ceiling, formant_shift_ratio,
               new_pitch_median, pitch_range_factor, duration_factor
```

For female-to-male: `formant_shift_ratio ≈ 1/1.1 = 0.91`, target median ≈ 120 Hz. For male-to-female: ratio ≈ 1.1, target ≈ 220 Hz.

### `Sound: Shift frequencies...`

All-pass spectral shift: moves every frequency by a fixed delta. Destroys harmonic relationships; useful for special-effect processing but not for natural vocal transformation. Parameters: shift frequency (Hz), max frequency (Hz).

### All-pass formant shifting

Available indirectly: extract Formant object, modify its internal table, use `Sound & Formant: Filter` to reshape the source with modified tract. This is manual but preserves source separately from filter.

## TextGrid alignment

TextGrid is Praat's annotation format. A TextGrid has:

- **IntervalTier** — a partition of `[xmin, xmax]` into labelled intervals (boundaries + text labels). Used for phoneme, word, or phrase segmentation.
- **PointTier** — labelled time points. Used for beat markers, onsets, creak marks.

TextGrid files are plain text (`.TextGrid` extension, UTF-8 with BOM in some versions — be aware). There is also a short "short" format and a binary `.Collection` representation.

Praat does **not** ship forced alignment. The canonical workflow is:

1. Run Montreal Forced Aligner (MFA) or Aeneas externally.
2. Import the resulting TextGrid into Praat for manual correction / inspection.
3. In Praat, use the TextGridEditor (GUI) or scripts to edit boundary times.

For our pipeline, TextGrid is the exchange format between MBROLA phoneme timing, vocal alignment stages, and Praat-based retargeting scripts. Example operations we need:

```praat
textgrid = Read from file: "take.TextGrid"
selectObject: textgrid
numIntervals = Get number of intervals: 1
for i from 1 to numIntervals
    label$ = Get label of interval: 1, i
    tstart = Get start time of interval: 1, i
    tend = Get end time of interval: 1, i
    appendInfoLine: label$, tab$, tstart, tab$, tend
endfor
```

## Spectral tools

Praat's spectral analysis tooling is comprehensive:

- **`Sound: To Spectrogram...`** — STFT. Parameters: window length (default 0.005 s for narrow-band, 0.03 s for broad-band), max frequency, time step, frequency step, Gaussian window. Broad-band (short window) resolves formants; narrow-band (long window) resolves harmonics.
- **`Sound: To Spectrum...`** — single-slice FFT spectrum (no time axis).
- **`Sound: To LPC (burg)...`** — LPC coefficient track; convert to Formant, Spectrogram, or Spectrum.
- **`Sound: To MFCC...`** — Mel-frequency cepstral coefficients; useful for timbre-similarity work.
- **`Sound: To PowerCepstrogram...`** — time-varying cepstrum; used for CPPS (cepstral peak prominence smoothed), a voice-quality metric.
- **`Sound: To Ltas...`** — long-term average spectrum. Use for voice fingerprinting or for characterising the spectral envelope of an entire take.
- **`Spectrum: Cepstral smoothing...`** — smooths a Spectrum via cepstral liftering to get a spectral envelope without LPC.

All these return objects that serialise cleanly to Table (tab-delimited) via `Down to Table...` or `Save as text file...`.

## Synthesis primitives

Praat can *generate* audio from first principles, which is useful for testing and for synthetic textures.

- **`Create Sound from formula...`** — evaluate a numeric expression of `x` (time) per sample. Example:
  ```praat
  Create Sound from formula: "tone", 1, 0, 2, 44100,
      ... "0.3 * sin(2*pi*440*x) + 0.2 * sin(2*pi*660*x)"
  ```
- **`Create Sound as pure tone...`** — single sinusoid with parameters (amplitude, frequency, ramp times).
- **`Create Sound from tone complex...`** — additive sinusoids, jitter/shimmer capable.
- **`Create KlattGrid...`** — the source-filter model as a time-varying parametric synthesizer.

### KlattGrid primer

A KlattGrid is a time-varying Klatt synthesizer. It has multiple tiers grouped into:

- **Phonation** — pitch, voicing amplitude, open phase, Power1/Power2 (glottal pulse shape), spectral tilt, aspiration, breathiness, double pulsing.
- **Vocal tract** — oral formants (as FormantGrid), nasal formants, nasal antiformants.
- **Coupling** — tracheal formants/antiformants, delta-formants.
- **Frication** — frication formants, bypass, noise source.
- **Output** — final gain.

Workflow:

```praat
klatt = Create KlattGrid: "texture", 0, 3, 6, 1, 1, 6, 1, 1, 1
Add pitch point: 0, 120
Add pitch point: 1.5, 180
Add pitch point: 3, 100
Add voicing amplitude point: 0, 80
Add oral formant frequency point: 1, 0, 700
Add oral formant bandwidth point: 1, 0, 80
Add oral formant frequency point: 2, 0, 1200
Add oral formant bandwidth point: 2, 0, 90
sound = To Sound
```

We use KlattGrid **sparingly**: it is easy to produce uncanny-valley synthetic voice. It is more useful as a source of *non-vocal* drones with animated resonances.

## Integration patterns

Our pipeline treats Praat as a stateless subprocess:

1. **Author script once.** Scripts live in `python/nadir-praat-scripts/scripts/*.praat`, checked in, pinned.
2. **Generate or pass inputs.** Rust callers stage WAV + params into a temp dir.
3. **Invoke Praat.** `praat --no-pref-files --run script.praat in.wav out.wav 220 1.0`.
4. **Parse outputs.** Scripts emit tab-delimited Tables to stdout or to `.Table` files via `Save as tab-separated file:`. Audio writes to WAV.
5. **Cleanup.** `select all` + `Remove` at the end of every script.

### Dynamic script generation

For one-off complex manipulations, we generate the script from Rust via a small template engine. The script is written to a temp file then invoked. This keeps the parametric surface in Rust and avoids fighting Praat's form syntax.

### Output parsing

Table objects serialise via `Save as tab-separated file: path$`. First row is column names, subsequent rows are values. Empty cells contain `--undefined--` — parse defensively.

For `appendInfoLine` output: write one record per line, tab-separated. Parse stdout with `csv` module in Python or `csv::ReaderBuilder::new().delimiter(b'\t')` in Rust.

### Binary `.Collection` files

`Save as binary file: path$` dumps the object stack as a `.Collection` — useful for caching intermediate Manipulation objects (pitch+pulse detection is expensive; reuse across scripts).

## Known pitfalls

- **PSOLA artefacts on short segments.** Pulses within ~2 periods of either edge get mismatched windows. Always pad audio with 100 ms of silence pre- and post-, and trim after resynthesis.
- **Pitch doubling / halving.** Autocorrelation can drop or jump an octave, especially at voiced-unvoiced boundaries or on sustained vowels with strong second-harmonic energy. Tighten `pitch_floor`/`pitch_ceiling` to plausible range and increase `octave_jump_cost` (default 0.35; try 0.5–0.7 for music).
- **Formant tracking instability on synthetic signals.** KlattGrid-generated or otherwise synthetic signals can throw off Burg's LPC pole solver. Use `To Formant (robust)` for KlattGrid output if you must track it.
- **Preferences file interference.** Praat reads `~/Library/Preferences/Praat Prefs/prefs5.ini` (macOS) and applies last-used analysis defaults. A previous GUI session can silently change behaviour. **Always pass `--no-pref-files` in scripted contexts.**
- **Locale-dependent decimal separators.** On systems with a comma decimal locale, Praat sometimes writes `3,14` to Tables instead of `3.14` when saving. Force `LC_NUMERIC=C` in the subprocess environment.
- **Command-name drift across versions.** Some commands were renamed across 6.x minor versions (e.g. `Get pitch` variants). Pin Praat version in `rust-toolchain`-style config and detect via `praat --version` at build time.
- **TextGrid encoding.** Some tools produce UTF-16 BOM TextGrids, some UTF-8. Praat accepts both but our downstream Rust parsers must too.
- **`noprogress` is required** in long scripts under `--run` on some builds, otherwise the script may hang waiting for GUI event loop.
- **Object IDs are process-local.** Never persist them across script invocations.

## How we use it in Nadir_Singleton

Praat is the PSOLA and formant layer of the vocal pipeline:

1. **MBROLA output pitch retargeting.** MBROLA produces WAV at a rigid F0 contour derived from its `.pho` input. We run `psola_retarget.praat` to bend the F0 to a scale-locked target contour generated by our music-theory layer, keeping formants (hence vowel identity) untouched.
2. **Formant character shifts.** For voice differentiation across tracks in a song, `formant_shift.praat` applies `Change gender`-style shifts (formant ratio 0.85–1.25) without altering pitch. Used to build "choir"-like textures from a single MBROLA voice.
3. **TextGrid phoneme timing edits.** MBROLA's `.pho` input determines phoneme durations; after synthesis, we use Praat TextGrid alignment (generated from the `.pho` file) to micro-adjust timings against the beat grid. `duration_warp.praat` consumes a target TextGrid and stretches each phoneme interval to its new bounds.
4. **KlattGrid non-vocal textures.** Drones, whooshes, formant-animated pads. Never used as a primary voice; always as background sonic material. `klatt_texture.praat` reads a JSON-ish parameter spec (passed as script args) and produces a single stem.
5. **Pitch analysis for style tracking.** `pitch_extract.praat` emits an F0 Table which our Rust side reads to drive auto-harmonising and melodic-contour analysis of reference takes.

Praat runs as a subprocess under our `nadir-vocal` crate; scripts are invoked by name, cache keys are SHA256 of input WAV + script + args.

## Reference script library (stubs)

The `python/nadir-praat-scripts` package ships the following `.praat` files. Each is self-contained, uses `--no-pref-files`-safe idioms, and follows the load/manipulate/resynth/save/cleanup pattern.

- **`psola_retarget.praat`** — inputs: input WAV, output WAV, target median Hz, range factor, optional target PitchTier file. Retargets F0 via PSOLA while holding formants. (Full example given above.)
- **`formant_shift.praat`** — inputs: input WAV, output WAV, formant shift ratio, target pitch median (0 = unchanged), duration factor. Thin wrapper around `Change gender` with explicit pitch floor / ceiling.
- **`pitch_extract.praat`** — inputs: input WAV, output Table path, pitch floor, pitch ceiling, time step. Runs `To Pitch (ac)`, writes tab-separated Table with columns `time`, `f0_hz`, `voiced` (0/1).
- **`duration_warp.praat`** — inputs: input WAV, input TextGrid, target TextGrid, output WAV. Per-interval time-stretch via DurationTier anchored at TextGrid boundaries; uses PSOLA.
- **`klatt_texture.praat`** — inputs: output WAV, duration seconds, pitch points JSON, formant tracks JSON (F1..F5 as time-value pairs), voicing amplitude envelope. Synthesises a KlattGrid texture and renders.

Each stub exposes a stable CLI (form fields in fixed order) so Rust callers can compose them without parsing Praat script syntax.

## References

Primary manual:

- Praat homepage: https://www.fon.hum.uva.nl/praat/
- Full manual index: https://www.fon.hum.uva.nl/praat/manual/
- Scripting tutorial: https://www.fon.hum.uva.nl/praat/manual/Scripting.html
- Calling from command line: https://www.fon.hum.uva.nl/praat/manual/Scripting_6_9__Calling_from_the_command_line.html
- Manipulation object: https://www.fon.hum.uva.nl/praat/manual/Manipulation.html
- Pitch analysis (AC): https://www.fon.hum.uva.nl/praat/manual/Sound__To_Pitch___.html
- Formant analysis (Burg): https://www.fon.hum.uva.nl/praat/manual/Sound__To_Formant__burg____.html
- Change gender: https://www.fon.hum.uva.nl/praat/manual/Sound__Change_gender___.html
- KlattGrid: https://www.fon.hum.uva.nl/praat/manual/KlattGrid.html
- TextGrid: https://www.fon.hum.uva.nl/praat/manual/TextGrid.html
- GitHub (docs mirror): https://github.com/praat/praat.github.io
- GitHub (source): https://github.com/praat/praat

Canonical papers:

- Boersma, P. (1993). *Accurate short-term analysis of the fundamental frequency and the harmonics-to-noise ratio of a sampled sound.* IFA Proceedings 17. The AC pitch algorithm.
- Boersma, P. & Weenink, D. (2001). *Praat, a system for doing phonetics by computer.* Glot International 5:9/10, 341–345. The original tool description.
- Hermes, D. J. (1988). *Measurement of pitch by subharmonic summation.* JASA 83(1). The SHS algorithm.
- Klatt, D. H. & Klatt, L. C. (1990). *Analysis, synthesis, and perception of voice quality variations among female and male talkers.* JASA 87. Klatt synthesizer basis for KlattGrid.
- Moulines, E. & Charpentier, F. (1990). *Pitch-synchronous waveform processing techniques for text-to-speech synthesis using diphones.* Speech Communication 9. The PSOLA technique.

How to cite in academic contexts: https://www.fon.hum.uva.nl/praat/manual/FAQ__How_to_cite_Praat.html
