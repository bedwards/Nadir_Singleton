#!/usr/bin/env bash
# End-to-end release pipeline: render → compile → master → video → schedule.
# Usage:
#   scripts/pipeline_end_to_end.sh [start_album_num] [end_album_num]
# Defaults: render all 20 albums.
#
# Steps (each idempotent — can be re-run):
#  1. render every track in selected albums (nadir album render)
#  2. compile into 11.5-min compilations (nadir compile)
#  3. master each compilation to -9 LUFS (nadir master)
#  4. encode MP4 video per mastered compilation (nadir video)
#  5. write YouTube schedule (nadir schedule)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="$PWD/tools/bin:$PATH"
NADIR="$ROOT/target/release/nadir"
[ -x "$NADIR" ] || cargo build --release --quiet

start=${1:-1}
end=${2:-20}

echo "=== 1/5 render ==="
for n in $(seq -f "%02g" "$start" "$end"); do
    album_dir=$(ls -d albums/${n}_* 2>/dev/null || true)
    [ -n "$album_dir" ] || continue
    album_slug=$(basename "$album_dir")
    echo "-- render $album_slug"
    "$NADIR" album render "$album_slug" --only-with-lyrics --keep-going || true
done

echo "=== 2/5 compile ==="
"$NADIR" compile --target-minutes 11.5 --out-dir compilations

echo "=== 3/5 master ==="
mkdir -p compilations/master
for wav in compilations/*.wav; do
    [ "$(basename "$wav" .wav)" = "manifest" ] && continue
    out="compilations/master/$(basename "$wav")"
    [ -f "$out" ] && continue
    "$NADIR" master "$wav" "$out"
done

echo "=== 4/5 video ==="
mkdir -p compilations/video
for wav in compilations/master/*.wav; do
    base=$(basename "$wav" .wav)
    out="compilations/video/${base}.mp4"
    [ -f "$out" ] && continue
    "$NADIR" video "$wav" "$out" --title "Nadir_Singleton — ${base}"
done

echo "=== 5/5 schedule ==="
"$NADIR" schedule --count 100 --out output/schedule.json

echo "=== done. ==="
ls -la compilations/video/ | head -20
echo "schedule: output/schedule.json"
