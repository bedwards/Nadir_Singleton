"""CLI for nadir-vad, invoked by the Rust `nadir` binary as a subprocess."""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from .segmenter import onsets, onsets_as_json, segments, segments_as_json


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="nadir-vad")
    sub = p.add_subparsers(dest="cmd", required=True)

    seg = sub.add_parser("segments", help="detect speech segments")
    seg.add_argument("--input", type=Path, required=True)
    seg.add_argument("--threshold", type=float, default=0.3)
    seg.add_argument("--min-speech-ms", type=int, default=60)
    seg.add_argument("--min-silence-ms", type=int, default=100)

    ons = sub.add_parser("onsets", help="detect onset grid")
    ons.add_argument("--input", type=Path, required=True)
    ons.add_argument("--threshold", type=float, default=0.3)
    ons.add_argument("--bpm", type=float, default=None)

    args = p.parse_args(argv)
    if args.cmd == "segments":
        out = segments_as_json(
            segments(
                args.input,
                threshold=args.threshold,
                min_speech_ms=args.min_speech_ms,
                min_silence_ms=args.min_silence_ms,
            )
        )
    elif args.cmd == "onsets":
        out = onsets_as_json(
            onsets(args.input, threshold=args.threshold, bpm=args.bpm)
        )
    else:
        p.print_help()
        return 2
    sys.stdout.write(json.dumps(out, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
