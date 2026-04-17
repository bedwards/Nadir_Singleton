"""ARPAbet → SAMPA mappings per MBROLA voice.

Sources: MBROLA us1 / us3 phoneme charts, IPA↔SAMPA reference, CMUdict ARPAbet
(stress digits are stripped before lookup).
"""
from __future__ import annotations

# us1 / us3 (American English). Sufficient for Albums 01-20 opening voice roster.
US_ARPABET_TO_SAMPA: dict[str, str] = {
    # vowels
    "AA": "A",
    "AE": "{",
    "AH": "V",
    "AO": "O",
    "AW": "aU",
    "AY": "aI",
    "EH": "E",
    "ER": "3r",
    "EY": "eI",
    "IH": "I",
    "IY": "i",
    "OW": "oU",
    "OY": "OI",
    "UH": "U",
    "UW": "u",
    # consonants
    "B": "b", "CH": "tS", "D": "d", "DH": "D",
    "F": "f", "G": "g", "HH": "h", "JH": "dZ",
    "K": "k", "L": "l", "M": "m", "N": "n",
    "NG": "N", "P": "p", "R": "r", "S": "s",
    "SH": "S", "T": "t", "TH": "T", "V": "v",
    "W": "w", "Y": "j", "Z": "z", "ZH": "Z",
}

VOICE_MAPPINGS = {
    "us1": US_ARPABET_TO_SAMPA,
    "us2": US_ARPABET_TO_SAMPA,
    "us3": US_ARPABET_TO_SAMPA,
}


def arpabet_to_sampa(tokens: list[str], voice: str = "us1") -> list[str]:
    m = VOICE_MAPPINGS.get(voice, US_ARPABET_TO_SAMPA)
    out: list[str] = []
    for t in tokens:
        bare = "".join(ch for ch in t if not ch.isdigit())
        if bare in m:
            mapped = m[bare]
            # Split multi-symbol SAMPA like "aU" into a single token; MBROLA
            # treats diphthongs as distinct phonemes — use as-is.
            out.append(mapped)
    return out
