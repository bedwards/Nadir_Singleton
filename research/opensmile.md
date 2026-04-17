# openSMILE — Research note (April 2026)

## What it is

openSMILE (recursively: the Munich **open-S**ource **M**edia **I**nterpretation by **L**arge feature-space **E**xtraction) is a C++ audio feature extraction toolkit designed for paralinguistic, prosodic, and musical analysis. It emits a large, standardised set of low-level descriptors (LLDs) and aggregated statistical functionals (mean, stddev, percentiles, slopes, etc.) per utterance or per frame. It is the de-facto reference extractor behind the Interspeech ComParE challenges, the Geneva Minimalistic Acoustic Parameter Set (GeMAPS/eGeMAPS), and a long line of emotion/speaker recognition research.

Origin: the project was started in 2008 by Florian Eyben, Martin Wöllmer, and Björn Schuller at **TU München** inside the EU-FP7 SEMAINE project, where it served as the acoustic emotion engine of a real-time dialogue system. First public release shipped with openEAR at ACII 2009; v1.0 was presented at ACM Multimedia 2010. From v2.1 onward it has been maintained by **audEERING GmbH** (Munich, Germany). It is dual-licensed — free for research, education, and personal use; a commercial license is required for products.

## Installation

### Current versions (verified April 2026)

- `opensmile` (C++ core) — **3.0.2** (tagged 2024-10-19). 3.0.2 is the first release with pre-built macOS arm64 (M1/M2/M3) and Linux ARM binaries.
- `opensmile` (Python package on PyPI) — **2.6.0** (released 2025-07-31). Requires Python >= 3.9, 64-bit only. Bundles the C++ core as a shared library; no separate SMILExtract install needed when using the Python wrapper.

There has been no 3.0.3 or 2.7.0 release as of April 2026; 3.0.2 / 2.6.0 are current.

### Source build on macOS arm64

```bash
git clone --depth 1 --branch v3.0.2 https://github.com/audeering/opensmile.git
cd opensmile

# Xcode CLT + cmake >= 3.15
xcode-select -p >/dev/null || xcode-select --install
brew install cmake

# Edit build_flags.sh if you want PORTAUDIO / FFmpeg / OpenCV enabled.
# Default build is CPU-only, no GUI, no portaudio.
bash build.sh

# Resulting binary:
./build/progsrc/smilextract/SMILExtract -h
```

If you prefer to drive CMake directly (recommended for CI):

```bash
cmake -S . -B build -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_OSX_ARCHITECTURES=arm64 \
  -DBUILD_FLAGS="-static -O2" \
  -DWITH_PORTAUDIO=OFF \
  -DWITH_FFMPEG=OFF \
  -DWITH_OPENCV=OFF \
  -DWITH_PYTHON=OFF
cmake --build build -j
```

Useful CMake flags: `WITH_PORTAUDIO`, `WITH_FFMPEG`, `WITH_OPENCV`, `WITH_OPENSLES` (Android), `BUILD_LIBOPENSMILE` (produces `libopensmile.dylib` plus `smileapi.h` for embedding).

Install the binary and the shipped `config/` tree somewhere known:

```bash
install -m755 build/progsrc/smilextract/SMILExtract /usr/local/bin/SMILExtract
mkdir -p /usr/local/share/opensmile && cp -R config /usr/local/share/opensmile/
export OPENSMILE_CONFIG_DIR=/usr/local/share/opensmile/config
```

### Python via uv (project-standard in Nadir_Singleton)

```bash
uv add opensmile           # adds the wheel with the bundled C++ core
uv add audiofile soundfile # recommended companions for I/O
```

The wheel on PyPI includes the C++ shared library and the full `config/` tree, so you do not need to install the `opensmile` repo separately unless you need a custom `.conf` that references components not shipped in the wheel.

### Release binaries

Pre-built `SMILExtract` binaries are published at `https://github.com/audeering/opensmile/releases` for:

- `opensmile-3.0.2-linux-x86_64`
- `opensmile-3.0.2-linux-armv8` (Raspberry Pi)
- `opensmile-3.0.2-macos-x86_64`
- `opensmile-3.0.2-macos-armv8` (M-series)
- `opensmile-3.0.2-win-x86_64`

Each archive ships the binary, the `config/` tree, and an `example-audio/opensmile.wav` fixture.

## Core binary: SMILExtract

`SMILExtract` is the single command-line entry point. It loads a `.conf` file which describes a directed processing graph, runs that graph against one input file (or stdin), and writes one or more output files.

Common invocation:

```bash
SMILExtract -C config/egemaps/v02/eGeMAPSv02.conf \
            -I input.wav \
            -csvoutput out.functionals.csv \
            -lldcsvoutput out.lld.csv \
            -instname "track_01"
```

Flag reference:

| Flag | Meaning |
|------|---------|
| `-C <file>` | Configuration file to load (required) |
| `-I <wav>` | Input wave file (binds `\cm[inputfile]`) |
| `-O <file>` | Default-format output file (often ARFF) |
| `-csvoutput <file>` | CSV file, functionals level |
| `-output <file>` | ARFF file, functionals level |
| `-lldcsvoutput <file>` | CSV file, one row per LLD frame |
| `-lldhtkoutput <file>` | HTK binary output, LLDs |
| `-lldarffoutput <file>` | ARFF output, LLDs |
| `-D <file>` | Shortcut: enable frame-wise LLD CSV |
| `-instname <str>` | Instance name embedded in ARFF/CSV |
| `-start <sec>` | Start time in input |
| `-end <sec>` | End time in input (-1 = EOF) |
| `-appendcsv 0/1` | Overwrite vs append CSV (default 1) |
| `-headercsv 0/1` | Emit CSV header row (default 1) |
| `-timestampcsv 0/1` | Emit frame timestamps (default 1) |
| `-l <0..9>` | Log verbosity (default 2) |
| `-logfile <path>` | Write logs to file |
| `-noconsoleoutput` | Silence stderr |
| `-nticks <n>` | Max processing ticks (-1 infinite) |
| `-h` | Usage |
| `-L` | List all compiled components |
| `-H <componentName>` | Print config help for a component |
| `-cfgFileTemplate -configDflt "cWaveSource,cFramer,..."` | Generate a starter `.conf` |

Output formats: **CSV** (default for most pipelines), **ARFF** (Weka native; carries class labels and attribute metadata), **HTK** (binary time-series for HMM tooling), and **raw LLD** streams. ARFF is preferable if you plan to ship to sklearn via `scipy.io.arff` because it carries dtype information.

## Config file anatomy

A `.conf` is an INI-like description of a *component graph*. Everything downstream is built from three abstract classes defined in the `cComponentManager`:

1. `cDataSource` — produces frames (e.g. `cWaveSource` reading a WAV, `cOpenSLESSource` for Android mic).
2. `cDataProcessor` — owns a `cDataReader` and a `cDataWriter` and transforms one level into another (e.g. `cFramer`, `cWindower`, `cTransformFFT`, `cFFTmagphase`, `cMelspec`, `cMfcc`, `cPitchACF`, `cFunctionals`).
3. `cDataSink` — terminal consumer (`cCsvSink`, `cArffSink`, `cHtkSink`, `cLibsvmSink`, `cOpenSLESSink`).

They exchange data through the shared `cDataMemory`, which hosts named *levels* — each a ring-buffer/matrix (N channels × infinite frames). **Exactly one writer, many readers.** Typical levels in a GeMAPS pipeline:

```
wave → frames → winframes → fftmag → spectral → pitch/voicing → lld → functionals
```

Each component block in the config names the level it reads from (`reader.dmLevel`) and the level it writes to (`writer.dmLevel`). Top-level `[componentInstances:cComponentManager]` lists every instance with its concrete type. The `\cm[name(cliFlag):description]` macro binds a placeholder to a command-line argument.

Minimal working config (logs RMS energy per 25 ms frame to CSV):

```ini
[componentInstances:cComponentManager]
instance[dataMemory].type = cDataMemory
instance[waveSource].type = cWaveSource
instance[framer].type     = cFramer
instance[energy].type     = cEnergy
instance[csvSink].type    = cCsvSink
printLevelStats = 1
nThreads = 1

[waveSource:cWaveSource]
writer.dmLevel = wave
filename = \cm[inputfile(I):input wave file]
monoMixdown = 1

[framer:cFramer]
reader.dmLevel = wave
writer.dmLevel = frames
frameSize = 0.025
frameStep = 0.010
frameMode = fixed

[energy:cEnergy]
reader.dmLevel = frames
writer.dmLevel = energy
rms = 1
log = 1

[csvSink:cCsvSink]
reader.dmLevel = energy
filename = \cm[outputfile(O):output CSV]
append = 0
```

Run with `SMILExtract -C minimal.conf -I in.wav -O out.csv`. The standard shipped configs (`config/egemaps/v02/eGeMAPSv02.conf`, `config/is09-13/IS13_ComParE.conf`, etc.) use the same grammar but include dozens of stages; they `\{include}` modular `.conf.inc` files under `config/shared/` to reuse the spectral front-end.

## Standard feature sets

All configs ship under `config/` in the source/wheel and can be referenced directly.

| Set | Config | LLDs | Functionals total | Typical use |
|-----|--------|------|-------------------|-------------|
| **GeMAPSv01a/b** | `config/gemaps/v01a/GeMAPSv01a.conf` | 18 | **62** | Minimal affect baseline |
| **eGeMAPSv01a/b** | `config/egemaps/v01a/…` | 23 | 88 | Extended GeMAPS |
| **eGeMAPSv02** | `config/egemaps/v02/eGeMAPSv02.conf` | 25 | **88** | Current default for emotion/voice analytics |
| **ComParE 2016** | `config/is09-13/IS13_ComParE.conf` | 65 | **6373** | High-dimensional paralinguistics (Interspeech challenge) |
| **emobase** | `config/emobase/emobase2010.conf` | 26 | ~1582 | Classic Berlin/emo-DB baseline (IS2010) |
| **IS09_emotion** | `config/is09-13/IS09_emotion.conf` | 16 | 384 | Interspeech 2009 Emotion Challenge |
| **IS10_paraling** | `config/is09-13/IS10_paraling.conf` | 38 | 1582 | IS 2010 Paralinguistic Challenge |
| **IS11_speaker_state** | `config/is09-13/IS11_speaker_state.conf` | 59 | 4368 | IS 2011 |
| **IS12_speaker_trait** | `config/is09-13/IS12_speaker_trait.conf` | 64 | 6125 | IS 2012 |
| **MFCC12_E_D_A** | `config/mfcc/MFCC12_E_D_A.conf` | 39 | — | ASR-style MFCC + Δ + ΔΔ |
| **PLP_E_D_A** | `config/plp/PLP_E_D_A.conf` | — | — | Perceptual linear prediction |
| **chroma_fft** | `config/chroma/chroma_fft.conf` | 12 | — | Music / key detection |
| **prosodyAcf / prosodyShs** | `config/prosody/*.conf` | — | — | Pitch + energy prosody only |

Frame timing conventions:

- **GeMAPS / eGeMAPS:** two parallel front-ends — a 20 ms / 10 ms Gaussian-windowed frame stream for pitch, jitter, shimmer, HNR, formants; a 25 ms / 10 ms Hamming-windowed frame stream for loudness, MFCCs, spectral features. Final functionals are computed over smoothed LLD contours with a 3-frame symmetric moving average and Viterbi voicing smoothing.
- **ComParE 2016:** 60 ms / 10 ms for pitch and voicing, 25 ms / 10 ms for everything else.
- **MFCC12_E_D_A / PLP_E_D_A:** 25 ms / 10 ms, pre-emphasis 0.97.

## LLD vs functionals

An LLD (low-level descriptor) is a value emitted once per analysis frame — e.g. an F0 estimate every 10 ms, a bark-band energy, the first MFCC. An LLD output is therefore a time-series `T × D` (frames by feature dimension).

A **functional** is a scalar statistic applied to an LLD contour over a segment (usually the whole utterance): arithmetic mean, standard deviation, coefficient of variation, 20th/50th/80th percentile, inter-quartile range, linear regression slope and offset, peak mean, rising-slope mean, voiced-segment duration, etc. A functional output is therefore a single row per utterance with a wide column count (62, 88, 6373).

How to pick the level:

```bash
# Functionals only (one row per file)
SMILExtract -C .../eGeMAPSv02.conf -I in.wav -csvoutput func.csv

# LLDs only (T rows per file)
SMILExtract -C .../eGeMAPSv02.conf -I in.wav -lldcsvoutput lld.csv -instname track

# Both
SMILExtract -C .../eGeMAPSv02.conf -I in.wav \
    -csvoutput func.csv -lldcsvoutput lld.csv
```

Some configs gate output with internal booleans (e.g. the ComParE config exposes `-lldoutput` / `-lldhtkoutput` but writes nothing unless you pass them). Use `-H cCsvSink` to inspect a specific sink's options.

## Musically relevant features

For music production — not just speech — the subset we actually pull is exposed most cleanly by **eGeMAPSv02**. Mapping feature to config:

- **F0 / pitch (semitone scale, 27.5 Hz reference)** — LLD `F0semitoneFrom27.5Hz_sma3nz`. eGeMAPSv02 uses subharmonic-summation (SHS) over a Gaussian-windowed 20 ms / 10 ms frame, followed by Viterbi path smoothing. Also available via `cPitchACF` (autocorrelation) in `prosodyAcf.conf`.
- **Voicing probability** — LLD `voicingFinalUnclipped_sma3nz` (0..1). Drives the `_nz` ("non-zero only") versions of every voiced statistic.
- **Jitter (local)** — `jitterLocal_sma3nz`. Cycle-to-cycle F0 period deviation. Very sensitive at SR ≥ 44.1 kHz and frameSize ≥ 60 ms.
- **Shimmer (local dB)** — `shimmerLocaldB_sma3nz`. Cycle-to-cycle amplitude deviation.
- **HNR (dB)** — `HNRdBACF_sma3nz`. Harmonics-to-noise ratio; high for clean tones, low for breathy/noisy sources.
- **Loudness (Zwicker, sone)** — `loudness_sma3`. Perceptual, auditory-spectrum-based. Prefer this over raw RMS for perceptual deltas.
- **MFCC 1–4** — `mfcc1_sma3` … `mfcc4_sma3`. Timbral coarse shape. (Full 13-coef MFCC via `MFCC12_E_D_A.conf`.)
- **Spectral flux** — `spectralFlux_sma3`. Frame-to-frame spectral change; a solid onset / attack proxy.
- **Spectral slope (0–500 Hz and 500–1500 Hz)** — `slope0-500_sma3`, `slope500-1500_sma3`. Tilt of log magnitude; proxy for brightness / source-filter balance.
- **Harmonic differences H1-H2, H1-A3** — `logRelF0-H1-H2_sma3nz`, `logRelF0-H1-A3_sma3nz`. Voice quality (breathy vs pressed).
- **Formants F1/F2/F3** — `F1frequency_sma3nz`, `F1bandwidth_sma3nz`, etc.
- **Alpha ratio (50–1000 Hz / 1–5 kHz)** — `alphaRatio_sma3`. Lump brightness measure.
- **Hammarberg index** — `hammarbergIndex_sma3`.

For **music-specific** timbral/harmonic work, supplement with:

- `config/chroma/chroma_fft.conf` — 12-D chroma per frame.
- `config/mfcc/MFCC12_E_D_A.conf` — full MFCC + Δ + ΔΔ.
- A custom config (see "Custom configs") adding `cSpectralCentroid`, `cRollOff`, `cFlatness`, `cTonalCentroid`.

## Integration patterns

**Piping via stdin.** `cWaveSource` accepts `filename = -`, in which case it reads a WAV stream from stdin. Combined with `ffmpeg -i in.m4a -f wav -`, this lets you extract features without writing temp files:

```bash
ffmpeg -v error -i take.m4a -ac 1 -ar 48000 -f wav - \
  | SMILExtract -C config/egemaps/v02/eGeMAPSv02.conf \
                -I - -csvoutput /dev/stdout -instname take
```

**ARFF parsing.** ARFF is preferable when you want typed attributes for free:

```python
from scipy.io import arff
import pandas as pd
data, meta = arff.loadarff("out.arff")
df = pd.DataFrame(data)
df["name"] = df["name"].str.decode("utf-8")
```

**CSV parsing.** CSV sinks use `;` as the default separator (audEERING legacy). Override with `csvSink.delimChar = ','` in your config, or parse with `sep=';'`:

```python
import pandas as pd
lld = pd.read_csv("out.lld.csv", sep=';')
# columns: name, frameTime, <feature_1>, ..., <feature_N>
```

**Render audit loop.** In practice, once you have a target LLD contour (e.g. a scale-snapped F0 curve), you re-render, re-extract, and compare the new F0 contour to the target. Perceptual thresholds we rely on:

- **RMS F0 error < 2 cents** — indistinguishable from target for non-vibrato contexts.
- **RMS F0 error 2–10 cents** — audible but musically acceptable.
- **RMS F0 error > 15 cents** — a listener will flag it as out of tune.
- **Loudness RMS delta < 0.5 sone** — perceptually invisible.
- **MFCC1 RMS delta < 0.3** — timbre preserved.
- **Spectral flux mean delta > 10%** — attack character has shifted.

These thresholds are the basis for feature-gated track acceptance.

## Python API

The `opensmile` Python package wraps `libopensmile` (the C++ core) via `ctypes`. No external `SMILExtract` binary is needed; the wheel bundles both the shared library and the shipped `config/` tree. The wrapper does not shell out to `SMILExtract`; it calls `smileapi` directly and returns a `pandas.DataFrame`.

```python
import opensmile
import numpy as np
import audiofile

# 1. Construct once per process (expensive: parses config, builds graph)
smile = opensmile.Smile(
    feature_set=opensmile.FeatureSet.eGeMAPSv02,
    feature_level=opensmile.FeatureLevel.Functionals,
)

# 2a. From file
df = smile.process_file("take.wav")
# DataFrame index = (file, start, end) as Timedelta
# DataFrame columns = 88 named functionals

# 2b. From numpy
signal, sr = audiofile.read("take.wav", always_2d=True)  # (C, N)
df = smile.process_signal(signal[0], sr)

# 2c. Batch
df = smile.process_files(["a.wav", "b.wav", "c.wav"])

# 2d. Via audformat index (supports num_workers for parallelism)
df = smile.process_index(index, num_workers=8)
```

`FeatureSet` enum members (as of opensmile-python 2.6.0):

```
ComParE_2016
GeMAPSv01a
GeMAPSv01b
eGeMAPSv01a
eGeMAPSv01b
eGeMAPSv02
emobase
```

`FeatureLevel` enum members:

```
LowLevelDescriptors        # per-frame (10 ms hop)
LowLevelDescriptors_Deltas # ComParE_2016 only
Functionals                # one row per file/segment
```

Constructor extras worth knowing:

- `channels=[0]` — pick a specific channel, or `[0, -1]` for left+right.
- `sampling_rate=48000, resample=True` — force a target sample rate (internally uses `soxr`).
- `num_workers` — on `process_index` / `process_files`, parallelises across files.
- `logfile="smile.log"`, `loglevel=3` — persist SMILExtract's internal logs.

Custom configs work by passing a path instead of an enum value:

```python
smile = opensmile.Smile(
    feature_set="my_config.conf",
    feature_level=opensmile.FeatureLevel.LowLevelDescriptors,
    options={"output_fields": "pitch,energy"},  # becomes \cm[...] substitutions
)
```

## Custom configs

Rough recipe for a custom feature vector:

1. Start from the nearest shipped config (e.g. `config/shared/FrameModeFunctionals.conf.inc`).
2. Identify the level of the LLD you want — e.g. spectral centroid on the `spectral` level.
3. Add a `cSpectralCentroid` instance reading `spectral`, writing `centroid`.
4. Wire a `cFunctionals` instance with the stats you want (`means`, `stddev`, `percentile 20 50 80`, `regression`, `peaks`) reading `centroid`.
5. Add a `cCsvSink` reading the functional level (or an LLD sink reading `centroid` directly).

Core component library to remember (`SMILExtract -L` prints the full list; 2026/3.0.2 ships ~120 components):

- Frontends: `cWaveSource`, `cFramer`, `cWindower`, `cPreemphasis`.
- Spectral: `cTransformFFT`, `cFFTmagphase`, `cMelspec`, `cBandspec`, `cSpecScale`.
- Cepstral: `cMfcc`, `cPlp`.
- Pitch: `cPitchACF`, `cPitchShs`, `cPitchJitter`, `cPitchSmootherViterbi`.
- Voice: `cFormantLpc`, `cFormantSmoother`, `cHarmonics`, `cSpectralHarmonics`, `cAcf`.
- Spectral statistics: `cSpectralCentroid` (via `cSpectral`), `cRollOff`, `cFlatness`, `cFlux`, `cEntropy`, `cSpectralSlope`, `cAlphaRatio`, `cHammarbergIndex`.
- Chroma / musical: `cChroma`, `cTonespec`, `cTonalCentroid`.
- Statistics: `cFunctionals`, `cFunctionalMoments`, `cFunctionalPeaks`, `cFunctionalSegments`.
- Sinks: `cCsvSink`, `cArffSink`, `cHtkSink`, `cLibsvmSink`.

For every component, `SMILExtract -H cComponentName` prints the full list of configurable fields.

## Known pitfalls

- **Sample rate handling.** The shipped speech configs expect 16 kHz. Feeding 44.1 or 48 kHz audio works (the front-end resamples internally via `cSpecResample` in the `.conf.inc` includes that reference it), but ComParE/IS configs assume 16 kHz mel-scale bands — if you care about MFCC/bandpower comparability across takes, downsample to 16 kHz **before** feeding them. Music-targeted configs (chroma, MFCC12_E_D_A) are sample-rate-agnostic because the mel/chroma banks rescale.
- **Short-clip artefacts.** Jitter/shimmer need ≥ ~3 full F0 periods inside the analysis window. On 60 Hz subjects at 60 ms that's ~3 cycles; on 440 Hz at 20 ms that's ~8 cycles (fine). Clips shorter than ~500 ms will have zero-variance functionals on slow-moving LLDs — you will get `NaN` / `0` in `stddev`, `slope`, `regression`. Filter.
- **Voiced/unvoiced on synthetic (MBROLA) input.** MBROLA output has perfectly clean, non-jittery voicing. `cPitchSmootherViterbi` may mark silent gaps as voiced or vice versa because its voicing-probability threshold is calibrated on human input. Remediation: raise `voicingCutoff = 0.55` (from the default 0.50) in a copied config, and post-filter voicing-cut transients < 30 ms.
- **Stereo handling.** `cWaveSource` defaults to `monoMixdown = 1` only in some configs. When it is 0, stereo files produce interleaved LLD frames that will silently corrupt functionals. Force mono in your config, or resample to mono with `ffmpeg -ac 1` before invocation, or use `channels=[0]` in the Python API.
- **CSV delimiter.** Default is `;` (semicolon), not `,`. Every downstream tool must know.
- **`instname`.** If you forget `-instname`, ARFF and CSV emit the input path as the instance identifier, which is fragile across rigs. Always set it explicitly.
- **Config tree path.** When using the CLI, relative `\{include}` paths in shipped configs are relative to the CWD, not the config. Run `SMILExtract` from `/usr/local/share/opensmile/` (or wherever you installed `config/`), or set absolute `\{include}` paths.
- **Version drift on eGeMAPS.** `v01a` and `v01b` differ in a handful of functional definitions; `v02` (current default) replaces several v01 features (notably 2 MFCC deltas) and smooths voicing differently. Pin to `eGeMAPSv02` in all new pipelines.

## How we use it in Nadir_Singleton

openSMILE is one of the two audit oracles in the pipeline (the other is Praat). Concrete touchpoints:

1. **F0 audit loop.** MBROLA renders a synthesis pass → `SMILExtract -C eGeMAPSv02.conf -I render.wav -lldcsvoutput lld.csv`. We read `F0semitoneFrom27.5Hz_sma3nz` + `voicingFinalUnclipped_sma3nz`, snap voiced frames to the track's scale, feed the target contour to **Praat PSOLA** (`Manipulation → Pitch tier → Replace`), then re-audit with openSMILE. The loop terminates when RMS pitch error < **2 cents** (our internal "perceptually clean" bar). Unvoiced frames are masked out before RMS.
2. **Feature-gated track acceptance.** Each candidate take is processed through eGeMAPSv02 functionals and scored against the per-track reference vector. Gates are:
   - RMS pitch delta < 2 cents (primary).
   - Loudness mean within ±0.5 sone of reference.
   - MFCC1 RMS delta < 0.3 (timbre preserved).
   - Spectral flux mean within ±10 % (attack character preserved).
   - Jitter local within 2× reference (prevents robotic smoothing).
   Takes failing any hard gate are sent back to the renderer with corrective hints.
3. **Cross-album motif fingerprinting.** Every motif clip (2–8 s) is passed through eGeMAPSv02 Functionals, producing an 88-D vector. Vectors are L2-normalised and indexed in a FAISS IndexFlatIP. At composition time, a proposed new motif is fingerprinted and nearest-neighbour searched against all prior album vectors; cosine similarity > 0.92 is flagged as "too close to prior motif X". This is cheap (< 50 ms per fingerprint on M2) and surprisingly discriminative for motifs of similar register and timbre.
4. **Stem QC.** ComParE 2016 LLDs (65-D) are logged per-frame for every stem; anomalous jitter / flux spikes trigger a human review before the master is committed.

## References

- **GitHub repository (C++ core):** https://github.com/audeering/opensmile
- **GitHub repository (Python wrapper):** https://github.com/audeering/opensmile-python
- **Documentation (core):** https://audeering.github.io/opensmile/
- **Documentation (Python):** https://audeering.github.io/opensmile-python/
- **PyPI package:** https://pypi.org/project/opensmile/
- **audEERING research page:** https://www.audeering.com/research/opensmile/
- **eGeMAPS paper** — F. Eyben, K. R. Scherer, B. W. Schuller, J. Sundberg, E. André, C. Busso, L. Y. Devillers, J. Epps, P. Laukka, S. S. Narayanan, K. P. Truong, "The Geneva Minimalistic Acoustic Parameter Set (GeMAPS) for Voice Research and Affective Computing," *IEEE Transactions on Affective Computing*, vol. 7, no. 2, pp. 190–202, 2016. DOI: 10.1109/TAFFC.2015.2457417. Preprint: https://sail.usc.edu/publications/files/eyben-preprinttaffc-2015.pdf
- **ComParE 2016 paper** — B. Schuller, S. Steidl, A. Batliner, J. Hirschberg, J. K. Burgoon, A. Baird, A. Elkins, Y. Zhang, E. Coutinho, K. Evanini, "The INTERSPEECH 2016 Computational Paralinguistics Challenge: Deception, Sincerity & Native Language," *Proc. Interspeech 2016*, pp. 2001–2005. PDF: https://www5.informatik.uni-erlangen.de/Forschung/Publikationen/2016/Schuller-TI2.pdf
- **Original openSMILE paper** — F. Eyben, M. Wöllmer, B. Schuller, "openSMILE — The Munich Versatile and Fast Open-Source Audio Feature Extractor," *Proc. ACM Multimedia 2010*, pp. 1459–1462.
- **openSMILE 2.0 / 3.0 paper** — F. Eyben, F. Weninger, F. Groß, B. Schuller, "Recent Developments in openSMILE, the Munich Open-Source Multimedia Feature Extractor," *Proc. ACM Multimedia 2013*, pp. 835–838.
