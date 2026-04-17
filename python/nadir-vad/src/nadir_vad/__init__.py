"""Nadir_Singleton Silero-VAD wrappers.

Public API:
    segments(wav_path, *, threshold, min_speech_ms, min_silence_ms) -> list[Segment]
    onsets(wav_path, *, threshold, bpm=None) -> list[Onset]
"""
from .segmenter import Onset, Segment, onsets, segments

__all__ = ["Segment", "Onset", "segments", "onsets"]
