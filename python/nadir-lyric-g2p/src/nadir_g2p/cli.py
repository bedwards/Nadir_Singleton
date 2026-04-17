from __future__ import annotations

import argparse
import json
import sys

from .lexicon import phonemize_lyric


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="nadir-g2p")
    p.add_argument("--voice", default="us1")
    p.add_argument("--text", required=True)
    args = p.parse_args(argv)
    out = phonemize_lyric(args.text, voice=args.voice)
    sys.stdout.write(json.dumps(out))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
