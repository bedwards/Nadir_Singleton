"""Nadir_Singleton grapheme-to-phoneme for MBROLA voices.

ARPAbet → SAMPA mapping is voice-specific (us1, us3, en1, …). We maintain one
table per voice in :mod:`nadir_g2p.mappings`, plus stress-aware duration hints
and a contraction expander for lyric preprocessing.
"""
from .lexicon import phonemize_lyric, phonemize_word, phonemize_word_with_stress
from .mappings import (
    arpabet_to_sampa,
    arpabet_to_sampa_with_stress,
    expand_contractions,
    stress_duration_multiplier,
)

__all__ = [
    "arpabet_to_sampa",
    "arpabet_to_sampa_with_stress",
    "expand_contractions",
    "phonemize_lyric",
    "phonemize_word",
    "phonemize_word_with_stress",
    "stress_duration_multiplier",
]
