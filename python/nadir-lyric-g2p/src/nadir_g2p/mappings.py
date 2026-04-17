"""ARPAbet → SAMPA mappings per MBROLA voice, with stress-aware duration hints.

We target MBROLA us1 / us3 SAMPA inventories. CMUdict emits ARPAbet phonemes
with trailing stress digits (0, 1, 2). We strip those digits for the symbol
lookup but expose them separately as duration multipliers so upstream synthesis
can lengthen stressed syllables.

Sources: MBROLA us1 / us3 phoneme charts, IPA↔SAMPA reference, CMUdict ARPAbet.
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
    "AY": "AI",
    "EH": "E",
    "ER": "r=",
    "EY": "EI",
    "IH": "I",
    "IY": "i",
    "OW": "@U",
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

# Common English contractions → expanded forms. We use this to pre-process
# lyrics before CMUdict lookup, since CMUdict coverage of contracted forms is
# inconsistent and MBROLA diphone concatenation benefits from explicit word
# boundaries.
_CONTRACTIONS: dict[str, str] = {
    "don't": "do not",
    "won't": "will not",
    "can't": "can not",
    "i'm": "i am",
    "it's": "it is",
    "we're": "we are",
    "they're": "they are",
    "you're": "you are",
    "isn't": "is not",
    "aren't": "are not",
    "doesn't": "does not",
    "didn't": "did not",
    "wouldn't": "would not",
    "couldn't": "could not",
    "shouldn't": "should not",
    "let's": "let us",
}


def stress_duration_multiplier(stress: int) -> float:
    """Return a duration multiplier for a CMUdict stress digit.

    We use these to scale the target phoneme duration we hand to MBROLA:

    - ``1`` (primary stress)   → ``1.2``
    - ``2`` (secondary stress) → ``1.05``
    - ``0`` (unstressed)       → ``0.85``
    - anything else (e.g. consonants with no stress marker) → ``1.0``
    """
    if stress == 1:
        return 1.2
    if stress == 2:
        return 1.05
    if stress == 0:
        return 0.85
    return 1.0


def _split_stress(token: str) -> tuple[str, int | None]:
    """Split an ARPAbet token into (bare_symbol, stress_digit_or_None)."""
    digits = "".join(ch for ch in token if ch.isdigit())
    bare = "".join(ch for ch in token if not ch.isdigit())
    if digits:
        try:
            return bare, int(digits[-1])
        except ValueError:
            return bare, None
    return bare, None


def arpabet_to_sampa(tokens: list[str], voice: str = "us1") -> list[str]:
    """Map ARPAbet tokens (stress-agnostic) to SAMPA symbols for the voice."""
    m = VOICE_MAPPINGS.get(voice, US_ARPABET_TO_SAMPA)
    out: list[str] = []
    for t in tokens:
        bare, _ = _split_stress(t)
        if bare in m:
            # MBROLA treats diphthongs like "aU" as a single phoneme token.
            out.append(m[bare])
    return out


def arpabet_to_sampa_with_stress(
    tokens: list[str], voice: str = "us1"
) -> list[tuple[str, float]]:
    """Map ARPAbet tokens to ``(sampa, duration_multiplier)`` pairs.

    Consonants (which carry no stress digit in CMUdict) get a neutral
    multiplier of ``1.0``. Vowels carry ``0/1/2`` digits; we convert those via
    :func:`stress_duration_multiplier`.
    """
    m = VOICE_MAPPINGS.get(voice, US_ARPABET_TO_SAMPA)
    out: list[tuple[str, float]] = []
    for t in tokens:
        bare, stress = _split_stress(t)
        if bare not in m:
            continue
        mult = stress_duration_multiplier(stress) if stress is not None else 1.0
        out.append((m[bare], mult))
    return out


def expand_contractions(lyric: str) -> str:
    """Expand common English contractions in a lyric line.

    We preserve the original casing of the leading character when possible,
    so ``"We're going"`` becomes ``"We are going"`` rather than
    ``"we are going"``. Any contraction not in our table is left untouched.
    """
    def _sub(match_word: str) -> str:
        low = match_word.lower()
        if low not in _CONTRACTIONS:
            return match_word
        expanded = _CONTRACTIONS[low]
        if match_word[:1].isupper():
            expanded = expanded[:1].upper() + expanded[1:]
        return expanded

    # Split on whitespace but keep it so we can rejoin faithfully.
    parts: list[str] = []
    for token in lyric.split(" "):
        # Strip trailing punctuation for lookup, preserve it on output.
        leading = ""
        trailing = ""
        core = token
        while core and not (core[0].isalpha() or core[0] == "'"):
            leading += core[0]
            core = core[1:]
        while core and not (core[-1].isalpha() or core[-1] == "'"):
            trailing = core[-1] + trailing
            core = core[:-1]
        parts.append(leading + _sub(core) + trailing)
    return " ".join(parts)
