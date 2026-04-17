# Silero-VAD — Research note (April 2026)

This note covers what future agents need to know to use Silero-VAD for
segmentation, onset detection, and rhythmic gating of synthetic vocal streams
(principally MBROLA output and csdr-processed stems). It is opinionated about
repurposing VAD for rhythmic analysis rather than plain speech-vs-silence.

## What it is

Silero-VAD is a small, pre-trained neural voice-activity-detection model by
the Silero team (`snakers4` on GitHub). It is MIT-licensed, has no telemetry
or registration, and ships as both TorchScript (JIT) and ONNX. The model is
~2 MB, small enough to embed in Rust crates, Python services, or WASM builds
without a large ML dependency tree.

It was trained on a large multilingual corpus (Silero claims 6000+
languages/dialects) and is the de-facto small-VAD across the open-source
speech stack (Whisper pre-processing, Piper, faster-whisper, LiveKit, Rhasspy).

As of April 2026 the current release is **v6.2.1** (PyPI, 2026-02-24). v6
uses an LSTM-plus-STFT architecture; it consumes 16 kHz or 8 kHz PCM and
emits a speech probability per 30 ms window. The v6 line is stable; pin
`silero-vad==6.2.*`. Python 3.8 through 3.15 are supported.

## Installation

Preferred (this repo uses uv):

```bash
uv add silero-vad
```

That pulls `silero-vad` from PyPI plus its minimal dependencies (`torch`,
`torchaudio`, `numpy`, and `onnxruntime` if you opt into the ONNX path).

Alternative entry points:

- **torch.hub** (no pip install; downloads model+utils on first call):
  ```python
  import torch
  model, utils = torch.hub.load(
      repo_or_dir="snakers4/silero-vad",
      model="silero_vad",
      trust_repo=True,
  )
  ```
  `utils` is a 5-tuple: `(get_speech_timestamps, save_audio, read_audio,
  VADIterator, collect_chunks)`.

- **ONNX-only path** (no torch at runtime; useful for Rust FFI or slim
  containers). Download `silero_vad.onnx` directly from the GitHub release
  assets (or copy from
  `site-packages/silero_vad/data/silero_vad.onnx` after a pip install) and
  load it with `onnxruntime`:
  ```bash
  curl -L -o silero_vad.onnx \
    https://github.com/snakers4/silero-vad/raw/master/src/silero_vad/data/silero_vad.onnx
  ```

- **Rust**: there is a community crate `silero-vad-rs` that wraps the ONNX
  model via `ort` (ONNX Runtime bindings). For Nadir_Singleton we prefer
  calling Python over PyO3 (see Integration patterns) to keep one source of
  truth for model handling.

## Model I/O contract

- **Sample rate**: 16 kHz or 8 kHz mono PCM, float32 in `[-1, 1]`. 16 kHz
  is default and what we use; 8 kHz is for telephony. Resample anything else.
- **Window size**: 512 samples at 16 kHz (= 32 ms) or 256 at 8 kHz. Silero
  keeps a 64-sample internal context, so feed exactly `window_size_samples`
  per call. The docs round this to "30 ms" but the math is 512/16000 = 32.
- **Output**: a float in `[0, 1]` per window = P(speech | window, history).
  Stateful (LSTM): feed windows in order; reset state between streams.
- **Thresholds**: speech-active when `p > threshold`; speech-ended when
  `p < neg_threshold` (default `max(0.5 - 0.15, 0.01) = 0.35`). Hysteresis
  prevents flapping.

## Python API

The pip package exposes a clean, high-level surface. Everything we do from
Python goes through these five names.

```python
from silero_vad import (
    load_silero_vad,       # returns a ready-to-use torch model
    get_speech_timestamps, # one-shot segmentation of a full waveform
    VADIterator,           # stateful streaming wrapper
    collect_chunks,        # concatenate timestamped spans back into audio
    read_audio,            # convenience wav reader (mono, target SR)
)
```

Full annotated example (live in `python/nadir_vad/cli.py`):

```python
# nadir_vad/cli.py — segmentation + onset extraction for mono 16 kHz wav.
# Usage: uv run python -m nadir_vad.cli input.wav --bpm 92 --threshold 0.35
from __future__ import annotations
import argparse, json, sys
from pathlib import Path

import numpy as np
import torch
from silero_vad import (
    load_silero_vad, read_audio, get_speech_timestamps,
    VADIterator, collect_chunks,
)

SR = 16_000
WINDOW = 512   # 32 ms at 16 kHz — the only legal window size
MOVING_AVG = 5 # 5 * 32 ms = 160 ms smoothing kernel


def segment(wav: torch.Tensor, threshold: float) -> list[dict]:
    """One-shot segmentation; list of {start, end} in seconds."""
    model = load_silero_vad(onnx=False)  # True for ONNX
    return get_speech_timestamps(
        wav, model,
        sampling_rate=SR,
        threshold=threshold,
        min_speech_duration_ms=120,   # MBROLA syllables are short
        min_silence_duration_ms=80,
        speech_pad_ms=30,
        return_seconds=True,
    )


def probability_track(wav: torch.Tensor) -> np.ndarray:
    """Raw per-window probability; get_speech_timestamps hides this."""
    model = load_silero_vad(onnx=False)
    model.reset_states()
    probs = []
    for i in range(0, len(wav) - WINDOW + 1, WINDOW):
        probs.append(model(wav[i : i + WINDOW], SR).item())
    return np.asarray(probs, dtype=np.float32)


def onsets_from_probs(probs, threshold, bpm):
    """Peak-pick rising edges; optionally quantize to BPM grid."""
    kernel = np.ones(MOVING_AVG, dtype=np.float32) / MOVING_AVG
    smooth = np.convolve(probs, kernel, mode="same")
    above = smooth > threshold
    rising = np.where(np.diff(above.astype(np.int8)) == 1)[0] + 1
    hop = WINDOW / SR
    onsets = [float(i * hop) for i in rising]
    if bpm is None:
        return onsets
    step = 60.0 / bpm / 4.0   # 16th-note grid
    return [round(o / step) * step for o in onsets]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("input", type=Path)
    ap.add_argument("--threshold", type=float, default=0.35)
    ap.add_argument("--bpm", type=float, default=None)
    a = ap.parse_args()

    wav = read_audio(str(a.input), sampling_rate=SR)
    probs = probability_track(wav)
    out = {
        "sample_rate": SR,
        "duration_s": len(wav) / SR,
        "threshold": a.threshold,
        "bpm": a.bpm,
        "segments": segment(wav, a.threshold),
        "onsets": onsets_from_probs(probs, a.threshold, a.bpm),
        "probability_hop_s": WINDOW / SR,
        "probability_track": probs.tolist(),
    }
    json.dump(out, sys.stdout)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

We deliberately run the model twice: once via `get_speech_timestamps` for
canonical segments (hysteresis + padding + min/max durations), and once
manually for the raw probability track (hidden behind the segmenter's
post-processing). `model.reset_states()` before the second pass is
critical — otherwise the LSTM carries state from the first run.

## get_speech_timestamps parameters

Canonical defaults (v6.2.x, verified against the source):

| Parameter                 | Default                                | Meaning |
|---------------------------|----------------------------------------|---------|
| `threshold`               | `0.5`                                  | Rising-edge probability for "speech on". |
| `neg_threshold`           | `max(threshold - 0.15, 0.01)` (`0.35`) | Falling-edge probability for "speech off". Provides hysteresis. |
| `sampling_rate`           | `16000`                                | Must be 8000 or 16000. |
| `min_speech_duration_ms`  | `250`                                  | Segments shorter than this are dropped. Lower for syllable-granularity. |
| `max_speech_duration_s`   | `float("inf")`                         | Splits long segments at internal silences. |
| `min_silence_duration_ms` | `100`                                  | Silence shorter than this is not counted as a gap. |
| `speech_pad_ms`           | `30`                                   | Pads each detected segment on both sides. |
| `window_size_samples`     | `512`                                  | Do not change from 512 (16 kHz) / 256 (8 kHz). |
| `return_seconds`          | `False`                                | If `True`, outputs seconds instead of samples. |
| `time_resolution`         | `1`                                    | Decimal precision when `return_seconds=True`. |

For Nadir_Singleton the defaults are too conservative. Synthetic MBROLA
syllables often run 120–200 ms, so we lower `min_speech_duration_ms` to 120
and `min_silence_duration_ms` to 60–80. We also drop `threshold` to 0.3–0.35
(see Behaviour on synthetic vocals).

## Streaming with VADIterator

`VADIterator` is the online wrapper. Signature:

```python
VADIterator(
    model,
    threshold: float = 0.5,
    sampling_rate: int = 16000,
    min_silence_duration_ms: int = 100,
    speech_pad_ms: int = 30,
)
```

Usage:

```python
from silero_vad import load_silero_vad, VADIterator, read_audio

SR, WINDOW = 16000, 512
model = load_silero_vad()
vad = VADIterator(model, sampling_rate=SR, threshold=0.35)

wav = read_audio("stream.wav", sampling_rate=SR)
for i in range(0, len(wav) - WINDOW + 1, WINDOW):
    event = vad(wav[i : i + WINDOW], return_seconds=True)
    if event:
        print(event)  # {'start': t} or {'end': t}

vad.reset_states()  # call between unrelated streams
```

Key points:
- The iterator only emits on transitions (start/end), not every window.
- `reset_states()` zeroes the LSTM state and the iterator's own
  `triggered`, `temp_end`, `current_sample` bookkeeping. Always call it
  before a new unrelated source.
- For live capture (PyAudio, sounddevice, cpal), feed exactly `WINDOW`
  samples per call — never half-windows. Ring-buffer upstream if your
  capture blocksize differs.

## ONNX runtime

For embedded / Rust / server use we prefer the ONNX path. Enable at load:

```python
model = load_silero_vad(onnx=True)
```

This constructs an `onnxruntime.InferenceSession` around the bundled
`silero_vad.onnx`. Useful session options:

```python
import onnxruntime as ort

opts = ort.SessionOptions()
opts.intra_op_num_threads = 1   # the model is tiny; 1 thread is fine
opts.inter_op_num_threads = 1
opts.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_ALL
sess = ort.InferenceSession(
    "silero_vad.onnx",
    sess_options=opts,
    providers=["CPUExecutionProvider"],  # or "CUDAExecutionProvider"
)
```

Performance: on a single x86 CPU thread, one 32 ms window takes under
1 ms (Silero claim; measured 0.2–0.6 ms on M2). That's 1000+ windows/sec
per core, i.e. 30+ seconds of audio per wall-second — plenty of headroom
for per-track rhythmic analysis.

CUDA works via `CUDAExecutionProvider` but is not worth it for a 2 MB
model — H2D/D2H transfer per 512-sample chunk dominates. Stick to CPU
unless you are batching huge numbers of streams.

## Behaviour on synthetic vocals

MBROLA is a diphone concatenative synthesiser. Two properties matter for VAD:

1. **Over-voiced**: no breath, no room tone, no inter-phoneme aspiration,
   uniformly high amplitude. Silero reads this as "almost always speech",
   which is technically correct but makes segments run together and loses
   syllable boundaries.
2. **Clean spectral envelope**: no noise floor, so probability rarely
   dips below ~0.2 even in intended gaps.

Tuning we've found robust on MBROLA:

- `threshold = 0.3` (sometimes 0.25) — accepts weak synthetic onsets that
  0.5 would merge into the preceding syllable.
- `neg_threshold = 0.12`, passed explicitly because the auto formula
  yields 0.15 which is too high for clean synthetic output.
- `min_silence_duration_ms = 60` to catch the ~80 ms inter-syllable gap.
- `min_speech_duration_ms = 100` to keep short consonant-only units.

Empirical tuning is mandatory. Build a fixture set of reference MBROLA
utterances, sweep `threshold` in 0.05 steps from 0.2 to 0.5, and pick the
value that minimises onset-count error vs human-labelled truth. Stash the
per-voice threshold in `albums/*/config.toml`.

## Repurposing VAD for rhythmic onset extraction

The Nadir_Singleton-specific trick: a VAD probability stream is a cheap,
surprisingly musical voicing envelope. We use it as a rhythm oracle.

Algorithm:

1. Run the model window-by-window, capturing `p[n]` at a 32 ms hop
   (512 samples at 16 kHz).
2. Smooth with a 5-window moving average (~160 ms kernel), removing
   single-window jitter without blurring syllable boundaries.
3. Detect rising edges: indices where `smooth[n-1] <= threshold` and
   `smooth[n] > threshold`. These are syllable onsets.
4. Convert to seconds: `t = n * 512 / 16000`.
5. Quantize to the BPM grid: `step = 60 / bpm / subdiv`,
   `t_q = round(t / step) * step`. `subdiv = 4` for 16ths, 8 for 32nds.
6. Emit the ordered onset list to the Rust sequencer.

Why it works: VAD probability is a learned function of speech-band energy
plus temporal coherence. On voiced material that tracks syllable nuclei
closely — exactly what we want for gridding. Cheaper than librosa
`onset_detect` on log-mel spectral flux, and more robust on clean
synthetic input where spectral-flux methods spuriously fire on sibilants
and formant transitions.

Caveat: the 32 ms hop caps onset precision at ±16 ms. Fine for 16ths up
to high tempos; for 32nds above ~160 BPM use a higher-resolution method.

## Segmentation for stem routing

Second use: partition a composite mix into voiced / unvoiced regions and
route each through a different DSP chain.

1. Downsample mix to 16 kHz mono (csdr `fir_decimate_cc` + channel
   collapse, or `sox -r 16000 -c 1`).
2. Run `get_speech_timestamps` for `[{start, end}]` segments.
3. For each segment, emit a csdr gate/envelope control track.
4. Bus voiced segments through one csdr chain (e.g. formant shimmer:
   `bandpass_fir_fft_cc`, `convert_f_s16`, `amdemod_cf`); unvoiced through
   another (reverb tail, noise duck).
5. Sum back at full 48 kHz. The 16 kHz pass is for routing masks only;
   actual audio is never destructively downsampled.

Use `collect_chunks(timestamps, wav)` to extract voiced material as a
contiguous tensor (vocal-only preview). For routing we keep timestamps.

## Integration patterns

From Rust we call Python via subprocess and parse JSON. Rust does not link
libpython in production; the CLI boundary keeps Python env churn out of
the Rust build.

Invocation:

```bash
uv run python -m nadir_vad.cli \
    /tmp/stem.wav \
    --bpm 92 \
    --threshold 0.3 \
    > /tmp/stem.vad.json
```

Output JSON schema (also the contract the Rust side `serde`-deserialises):

```json
{
  "sample_rate": 16000,
  "duration_s": 12.34,
  "threshold": 0.3,
  "bpm": 92.0,
  "segments": [
    {"start": 0.512, "end": 0.896},
    {"start": 1.152, "end": 1.472}
  ],
  "onsets": [0.489, 0.978, 1.141],
  "probability_hop_s": 0.032,
  "probability_track": [0.01, 0.02, 0.14, 0.78, 0.91, ...]
}
```

Rust-side the struct is `nadir_audio::VadReport`; `serde_json::from_reader`
off the child stdout is the idiomatic call site. For hot-path use (per-bar
live gating) a PyO3 binding with a long-lived interpreter avoids subprocess
spawn cost (~120 ms cold, ~30 ms warm via uv). Overkill for batch.

## Known pitfalls

- **Sample rate is load-bearing**. The model only accepts 8 or 16 kHz.
  Nadir_Singleton runs at 48 kHz; downsample with `csdr fir_decimate_cc 3`
  (48/3 = 16) or `sox -r 16000`. Do not use naive stride decimation — it
  aliases and Silero reads aliased audio as noise (chronic false negatives).
- **False positives on HF noise beds**. Broadband noise with speech-band
  energy (e.g. detuned square through a resonant LPF) can trigger the
  model. High-pass below 80 Hz before VAD, and raise `threshold` if the bed
  is intrinsically dense.
- **Threshold flapping**. Without hysteresis a stream near threshold will
  emit a storm of events. Keep `neg_threshold` >= 0.1 below `threshold` and
  `min_silence_duration_ms` >= 60.
- **Stateful model**. Failing to call `reset_states()` between unrelated
  audio carries LSTM state across and biases the first ~200 ms of the new
  stream. Shows up as a missed onset at the head of the file.
- **Window-size rigidity**. Trained on exactly 512 samples at 16 kHz.
  Feeding 480 samples (e.g. a 30 ms WebRTC frame) silently degrades
  accuracy; silero pads internally but padding skews the first-window
  probability. Resample so your blocksize divides 512.
- **No retraining for synthetic audio**. The model generalises to MBROLA,
  Piper, Festival, and coqui-TTS with threshold tuning only. Do not
  fine-tune — it regresses on eval and multilingual coverage collapses.
- **ONNX vs JIT drift**. Probabilities agree to ~1e-3 between runtimes,
  well below any threshold. A timestamp may shift by one 32 ms window.

## How we use it in Nadir_Singleton

1. **Rhythmic onset grid from vocal stem** — every MBROLA vocal stem goes
   through `nadir_vad.cli` with the song's BPM. The `onsets` array becomes
   the initial MIDI/event grid that drums, bass, and pads sync their
   triggers to, making the vocal the rhythmic spine, not an overlay.
2. **Silence-gating for DSP bed activation** — ambient beds (csdr drones,
   noise pads) subscribe to the `segments` list and duck by 6–9 dB during
   voiced regions, swelling back in gaps. Cheap ducking without a
   sidechain compressor.
3. **Pre-audit sanity check** — before rendering a full song we run VAD on
   a tiny preview bounce and assert `len(segments) > 0`. A silent stem
   (MBROLA misconfig, missing voice file) fails the build fast.
4. **Automatic section labelling** — long gaps in the probability track
   (`p < 0.1` sustained > 2 s) are section boundaries (verse/chorus/
   bridge). Labelled in the sidecar JSON and used to drive arrangement
   automation (filter sweeps, reverb throws, mute changes).

## References

- Repo: [snakers4/silero-vad](https://github.com/snakers4/silero-vad)
- PyPI: [silero-vad](https://pypi.org/project/silero-vad/) (v6.2.1, 2026-02-24)
- PyTorch Hub: [Silero VAD](https://pytorch.org/hub/snakers4_silero-vad_vad/)
- Wiki examples: [Examples and Dependencies](https://github.com/snakers4/silero-vad/wiki/Examples-and-Dependencies)
- Streaming notebook: [pyaudio-streaming-examples.ipynb](https://github.com/snakers4/silero-vad/blob/master/examples/pyaudio-streaming/pyaudio-streaming-examples.ipynb)
- Colab quickstart: [silero-vad.ipynb](https://colab.research.google.com/github/snakers4/silero-vad/blob/master/silero-vad.ipynb)
- ONNX asset: `src/silero_vad/data/silero_vad.onnx` in the repo (also shipped
  under `site-packages/silero_vad/data/` after pip install).
- License: MIT. Contact: hello@silero.ai, Telegram @silero_speech.

There is no canonical academic paper for v6; Silero treats release notes
and wiki as the reference. For LSTM-VAD background see the Google WebRTC
VAD papers and the 2018 Silero architecture blog posts linked from the
README.
