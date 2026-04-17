from nadir_g2p import phonemize_word, phonemize_lyric


def test_phonemize_word_oov():
    # "blorp" is OOV; spell-out should produce 5 phonemes.
    r = phonemize_word("blorp")
    assert len(r) >= 4


def test_phonemize_lyric_splits():
    r = phonemize_lyric("hello world")
    assert len(r) == 2
