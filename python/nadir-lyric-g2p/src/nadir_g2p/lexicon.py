from __future__ import annotations

import functools
import re

from .mappings import arpabet_to_sampa


@functools.lru_cache(maxsize=1)
def _cmudict():
    import cmudict  # type: ignore

    d = cmudict.dict()
    return d


def phonemize_word(word: str, voice: str = "us1") -> list[str]:
    """Return a list of SAMPA phonemes for `word` using CMUdict → mapping.

    Falls back to a crude spelling-based sound-out when OOV.
    """
    w = re.sub(r"[^a-zA-Z']", "", word).lower()
    if not w:
        return []
    entries = _cmudict().get(w)
    if entries:
        arp = entries[0]
        return arpabet_to_sampa(arp, voice=voice)
    return _fallback_spell_out(w, voice)


def _fallback_spell_out(word: str, voice: str) -> list[str]:
    table = {
        "a": ["{"], "e": ["E"], "i": ["I"], "o": ["O"], "u": ["V"],
        "b": ["b"], "c": ["k"], "d": ["d"], "f": ["f"], "g": ["g"],
        "h": ["h"], "j": ["dZ"], "k": ["k"], "l": ["l"], "m": ["m"],
        "n": ["n"], "p": ["p"], "q": ["k"], "r": ["r"], "s": ["s"],
        "t": ["t"], "v": ["v"], "w": ["w"], "x": ["k", "s"], "y": ["j"],
        "z": ["z"],
    }
    out: list[str] = []
    for ch in word:
        out.extend(table.get(ch, []))
    return out


def phonemize_lyric(lyric: str, voice: str = "us1") -> list[list[str]]:
    """Phonemize each whitespace-separated word → list of SAMPA phoneme lists."""
    return [phonemize_word(w, voice=voice) for w in lyric.split() if w.strip()]
