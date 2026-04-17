"""Robustness tests for Nadir_Singleton g2p.

We cover:

- stress-aware duration multipliers for CMUdict-backed words
- digraph handling in the OOV spell-out fallback
- contraction expansion for lyric preprocessing
- diphthong preservation (``aI``, ``aU``, ``OI``, ``oU``, ``eI``)
"""
from __future__ import annotations

from nadir_g2p import (
    expand_contractions,
    phonemize_lyric,
    phonemize_word,
    phonemize_word_with_stress,
)
from nadir_g2p.mappings import (
    arpabet_to_sampa_with_stress,
    stress_duration_multiplier,
)


def test_phonemize_lyric_dont_stop_believing_us1():
    """``don't stop believing`` → three words of sensible SAMPA on us1."""
    out = phonemize_lyric("don't stop believing", voice="us1")
    # "don't" expands to "do not" → 4 output words total.
    assert len(out) == 4
    # "do"
    assert out[0] == ["d", "u"]
    # "not"
    assert out[1] == ["n", "A", "t"]
    # "stop"
    assert out[2] == ["s", "t", "A", "p"]
    # "believing"
    assert out[3] == ["b", "I", "l", "i", "v", "I", "N"]


def test_phonemize_word_with_stress_believing_multipliers():
    """Primary/secondary/unstressed digits scale duration multipliers."""
    pairs = phonemize_word_with_stress("believing")
    # CMUdict "believing" → B IH0 L IY1 V IH0 NG. Mapping yields 7 phonemes.
    assert len(pairs) == 7
    sampa = [p for p, _ in pairs]
    assert sampa == ["b", "I", "l", "i", "v", "I", "N"]
    mults = [m for _, m in pairs]
    # Consonants neutral (1.0), IY1 primary (1.2), two IH0 unstressed (0.85).
    assert mults == [1.0, 0.85, 1.0, 1.2, 1.0, 0.85, 1.0]
    # Sum should sit near N * 1.0 — the stress scheme balances out.
    assert abs(sum(mults) - len(pairs) * 1.0) < 0.2


def test_phonemize_word_shout_has_S_and_diphthong_aU():
    """``shout`` starts with /ʃ/ (SAMPA ``S``) and contains the /aʊ/ diphthong."""
    phones = phonemize_word("shout")
    assert phones[0] == "S"
    assert "aU" in phones


def test_phonemize_word_through_starts_voiceless_dental_fricative():
    """``through`` starts with /θ/ (SAMPA ``T``)."""
    phones = phonemize_word("through")
    assert phones[0] == "T"


def test_expand_contractions_were_going():
    assert expand_contractions("we're going") == "we are going"


def test_expand_contractions_common_set():
    cases = {
        "don't": "do not",
        "won't": "will not",
        "can't": "can not",
        "I'm": "I am",
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
    for src, want in cases.items():
        assert expand_contractions(src) == want, src


def test_expand_contractions_preserves_punctuation_and_case():
    # Leading capital should carry through to the expansion's first word.
    assert expand_contractions("We're here.") == "We are here."
    # Non-contractions untouched.
    assert expand_contractions("hello world") == "hello world"


def test_stress_duration_multiplier_values():
    assert stress_duration_multiplier(1) == 1.2
    assert stress_duration_multiplier(2) == 1.05
    assert stress_duration_multiplier(0) == 0.85
    assert stress_duration_multiplier(9) == 1.0


def test_arpabet_to_sampa_with_stress_strips_digits():
    pairs = arpabet_to_sampa_with_stress(["AY1", "L", "AH0", "V"], voice="us1")
    assert pairs == [("aI", 1.2), ("l", 1.0), ("V", 0.85), ("v", 1.0)]


def test_diphthongs_single_sampa_tokens():
    """All five US diphthongs should emerge as single SAMPA tokens."""
    # price → P R AY1 S, mouth → M AW1 TH, choice → CH OY1 S,
    # goat → G OW1 T, face → F EY1 S.
    assert "aI" in phonemize_word("price")
    assert "aU" in phonemize_word("mouth")
    assert "OI" in phonemize_word("choice")
    assert "oU" in phonemize_word("goat")
    assert "eI" in phonemize_word("face")


def test_oov_digraph_fallback_sh():
    # "shnurg" is OOV; "sh" must collapse to a single /S/.
    phones = phonemize_word("shnurg")
    assert phones[0] == "S"


def test_oov_digraph_fallback_ch_th_ph_wh_ng_qu():
    # OOV nonsense words exercise each digraph.
    assert phonemize_word("chork")[0] == "tS"
    assert phonemize_word("thluz")[0] == "T"
    assert phonemize_word("phorg")[0] == "f"
    assert phonemize_word("whorp")[0] == "w"
    # "sing" is in CMUdict; use "zung"-style OOV to test ``ng``.
    phones_ng = phonemize_word("zrung")
    assert "N" in phones_ng
    phones_qu = phonemize_word("quorp")
    assert phones_qu[:2] == ["k", "w"]


def test_oov_silent_trailing_e():
    # "zike" is OOV; silent-e rule should drop trailing "e" from a CVC stem.
    phones = phonemize_word("zike")
    assert phones[-1] != "E"
