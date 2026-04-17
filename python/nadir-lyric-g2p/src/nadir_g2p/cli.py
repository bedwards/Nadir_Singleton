from __future__ import annotations

import argparse
import json
import sys

from .lexicon import phonemize_lyric, phonemize_lyric_with_stress


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="nadir-g2p")
    p.add_argument("--voice", default="us1")
    p.add_argument("--text", required=True)
    p.add_argument("--stress", action="store_true",
                   help="output stress weights alongside phonemes")
    args = p.parse_args(argv)
    if args.stress:
        out = phonemize_lyric_with_stress(args.text, voice=args.voice)
    else:
        out = phonemize_lyric(args.text, voice=args.voice)
    sys.stdout.write(json.dumps(out))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
