"""Silero-VAD segmentation + onset extraction.

We treat the VAD probability stream as an envelope that also marks syllable
onsets for rhythmic gating — a core Nadir_Singleton innovation in repurposing
a speech-detection model as a music-timing oracle.
"""
from __future__ import annotations

from dataclasses import asdict, dataclass
from pathlib import Path

import numpy as np
import soundfile as sf


@dataclass
class Segment:
    start_s: float
    end_s: float
    prob: float


@dataclass
class Onset:
    time_s: float
    prob: float
    beat_index: int | None = None


def _load_mono_16k(wav_path: Path) -> np.ndarray:
    data, sr = sf.read(str(wav_path), dtype="float32", always_2d=False)
    if data.ndim == 2:
        data = data.mean(axis=1)
    if sr != 16_000:
        # Linear-phase decimation via polyphase would be ideal, but we stay inside
        # the constraint: csdr would own that; here we fall back to simple numpy
        # only as an import-time necessity until the csdr upstream is wired.
        from math import gcd
        g = gcd(sr, 16_000)
        up, down = 16_000 // g, sr // g
        n = len(data)
        t_src = np.arange(n) / sr
        t_dst = np.arange(0, n * up // down) / 16_000
        data = np.interp(t_dst, t_src, data).astype("float32")
    return data


def _load_model():
    """Load the torch model by default; onnx=True requires the `onnx` extra."""
    from silero_vad import load_silero_vad

    try:
        return load_silero_vad(onnx=False)
    except TypeError:
        return load_silero_vad()


def segments(
    wav_path: Path,
    *,
    threshold: float = 0.3,
    min_speech_ms: int = 60,
    min_silence_ms: int = 100,
    speech_pad_ms: int = 30,
) -> list[Segment]:
    from silero_vad import get_speech_timestamps

    model = _load_model()
    audio = _load_mono_16k(Path(wav_path))
    ts = get_speech_timestamps(
        audio,
        model,
        sampling_rate=16_000,
        threshold=threshold,
        min_speech_duration_ms=min_speech_ms,
        min_silence_duration_ms=min_silence_ms,
        speech_pad_ms=speech_pad_ms,
        return_seconds=True,
    )
    return [Segment(start_s=float(x["start"]), end_s=float(x["end"]), prob=threshold) for x in ts]


def onsets(
    wav_path: Path,
    *,
    threshold: float = 0.3,
    bpm: float | None = None,
) -> list[Onset]:
    """Peak-pick where VAD probability rises across `threshold`.

    When `bpm` is given, each onset is snapped to the nearest 1/16-note grid and
    labelled with its beat index.
    """
    import torch

    model = _load_model()
    audio = _load_mono_16k(Path(wav_path))
    win = 512  # 32 ms at 16 kHz
    chunks = np.split(audio[: (len(audio) // win) * win], len(audio) // win)
    probs = []
    for c in chunks:
        with torch.no_grad():
            p = model(torch.from_numpy(c), 16_000).item()
        probs.append(p)
    probs = np.array(probs, dtype="float32")
    # Smooth 5-window moving average.
    kernel = np.ones(5, dtype="float32") / 5
    smoothed = np.convolve(probs, kernel, mode="same")
    # Detect rising edges across threshold.
    crossings = []
    was_below = True
    for i, v in enumerate(smoothed):
        t = (i * win) / 16_000
        if was_below and v >= threshold:
            crossings.append(Onset(time_s=float(t), prob=float(v)))
            was_below = False
        elif not was_below and v < threshold:
            was_below = True

    if bpm is not None and crossings:
        grid = (60.0 / bpm) / 4.0  # 1/16 note
        for o in crossings:
            bi = round(o.time_s / grid)
            o.beat_index = int(bi)
            o.time_s = float(bi * grid)
    return crossings


def split_segments(
    wav_path: Path,
    out_dir: Path,
    threshold: float = 0.3,
    min_speech_ms: int = 60,
    min_silence_ms: int = 100,
) -> list[Path]:
    """Write each speech segment to out_dir/{n:03}.wav. Returns paths written."""
    out_dir.mkdir(parents=True, exist_ok=True)
    segs = segments(wav_path, threshold=threshold,
                    min_speech_ms=min_speech_ms, min_silence_ms=min_silence_ms)
    audio, sr = sf.read(str(wav_path), dtype="int16", always_2d=False)
    written: list[Path] = []
    for i, s in enumerate(segs):
        start = int(s.start_s * sr)
        end = int(s.end_s * sr)
        p = out_dir / f"{i:03}.wav"
        sf.write(str(p), audio[start:end], sr, subtype="PCM_16")
        written.append(p)
    return written


def segments_as_json(segs: list[Segment]) -> list[dict]:
    return [asdict(s) for s in segs]


def onsets_as_json(ons: list[Onset]) -> list[dict]:
    return [asdict(o) for o in ons]
