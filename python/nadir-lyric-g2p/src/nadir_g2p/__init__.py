"""Nadir_Singleton grapheme-to-phoneme for MBROLA voices.

ARPAbet → SAMPA mapping is voice-specific (us1, us3, en1, …). We maintain one
table per voice in `mappings/`.
"""
from .lexicon import phonemize_word, phonemize_lyric

__all__ = ["phonemize_word", "phonemize_lyric"]
