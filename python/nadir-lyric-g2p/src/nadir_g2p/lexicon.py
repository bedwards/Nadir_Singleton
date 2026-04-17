from __future__ import annotations

import functools
import re

from .mappings import (
    arpabet_to_sampa,
    arpabet_to_sampa_with_stress,
    expand_contractions,
)


@functools.lru_cache(maxsize=1)
def _cmudict():
    import cmudict  # type: ignore

    return cmudict.dict()


def phonemize_word(word: str, voice: str = "us1") -> list[str]:
    """Return a list of SAMPA phonemes for ``word`` using CMUdict → mapping.

    We fall back to a digraph-aware spelling sound-out when OOV.
    """
    w = re.sub(r"[^a-zA-Z']", "", word).lower()
    if not w:
        return []
    entries = _cmudict().get(w)
    if entries:
        arp = entries[0]
        return arpabet_to_sampa(arp, voice=voice)
    return _fallback_spell_out(w, voice)


def phonemize_word_with_stress(
    word: str, voice: str = "us1"
) -> list[tuple[str, float]]:
    """Return ``(sampa, duration_multiplier)`` pairs for ``word``.

    We preserve CMUdict stress digits end-to-end so callers can lengthen
    primary-stressed vowels. OOV words fall back to the digraph-aware
    spell-out with a neutral ``1.0`` multiplier per phoneme.
    """
    w = re.sub(r"[^a-zA-Z']", "", word).lower()
    if not w:
        return []
    entries = _cmudict().get(w)
    if entries:
        arp = entries[0]
        return arpabet_to_sampa_with_stress(arp, voice=voice)
    return [(p, 1.0) for p in _fallback_spell_out(w, voice)]


# Digraphs we recognise in the OOV fallback. Ordered by length so the scanner
# can greedy-match the longer patterns first.
_DIGRAPHS: dict[str, list[str]] = {
    "sh": ["S"],
    "ch": ["tS"],
    "th": ["T"],
    "ph": ["f"],
    "wh": ["w"],
    "ng": ["N"],
    "qu": ["k", "w"],
}

_SINGLE: dict[str, list[str]] = {
    "a": ["{"], "e": ["E"], "i": ["I"], "o": ["O"], "u": ["V"],
    "b": ["b"], "c": ["k"], "d": ["d"], "f": ["f"], "g": ["g"],
    "h": ["h"], "j": ["dZ"], "k": ["k"], "l": ["l"], "m": ["m"],
    "n": ["n"], "p": ["p"], "q": ["k"], "r": ["r"], "s": ["s"],
    "t": ["t"], "v": ["v"], "w": ["w"], "x": ["k", "s"], "y": ["j"],
    "z": ["z"],
}

_VOWELS = set("aeiouy")
_CONSONANTS = set("bcdfghjklmnpqrstvwxz")


def _strip_silent_e(word: str) -> str:
    """Drop a trailing silent ``e`` after a CVC pattern (e.g. ``make``, ``hope``).

    We keep the ``e`` when the word is too short or when the penultimate
    letter is already a vowel (``see``, ``toe``) — those ``e`` vowels are
    pronounced.
    """
    if len(word) < 4 or not word.endswith("e"):
        return word
    c3, v2, c1, _ = word[-4], word[-3], word[-2], word[-1]
    if c3 in _CONSONANTS and v2 in _VOWELS and c1 in _CONSONANTS:
        return word[:-1]
    return word


def _fallback_spell_out(word: str, voice: str) -> list[str]:
    """Crude letter-to-sound fallback for OOV words.

    Recognises common English digraphs (``sh``, ``ch``, ``th``, ``ph``, ``wh``,
    ``ng``, ``qu``) and drops a trailing silent ``e`` after a CVC pattern.
    """
    w = _strip_silent_e(word)
    out: list[str] = []
    i = 0
    while i < len(w):
        pair = w[i:i + 2]
        if pair in _DIGRAPHS:
            out.extend(_DIGRAPHS[pair])
            i += 2
            continue
        out.extend(_SINGLE.get(w[i], []))
        i += 1
    return out


def phonemize_lyric(lyric: str, voice: str = "us1") -> list[list[str]]:
    """Phonemize each whitespace-separated word → list of SAMPA phoneme lists.

    We expand common English contractions (``don't`` → ``do not``) before
    lookup so MBROLA receives discrete word boundaries.
    """
    expanded = expand_contractions(lyric)
    return [phonemize_word(w, voice=voice) for w in expanded.split() if w.strip()]
