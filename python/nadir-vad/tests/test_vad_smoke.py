from nadir_vad import Segment, Onset


def test_segment_dataclass():
    s = Segment(start_s=0.0, end_s=1.0, prob=0.5)
    assert s.start_s == 0.0


def test_onset_dataclass():
    o = Onset(time_s=0.25, prob=0.8, beat_index=4)
    assert o.beat_index == 4
