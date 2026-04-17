#!/usr/bin/env bash
# check_tools.sh — probe each of the five core tools; exit non-zero if any
# are missing. Used by CI to gate the verify job and by developers to
# sanity-check a fresh clone after scripts/bootstrap.sh.
#
# Detection order for each native tool:
#   1. absolute path under tools/bin/
#   2. command on PATH
#
# Silero-VAD is a Python package; we probe for the python/nadir-vad project
# directory since uv sync is what actually installs it.

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/tools/bin"
VOICES="$ROOT/tools/mbrola-voices"
OPENSMILE_CFG="$ROOT/tools/opensmile/config"

missing=0

say_ok()   { printf '  [ok]   %-14s %s\n' "$1" "$2"; }
say_fail() { printf '  [fail] %-14s %s\n' "$1" "$2"; missing=$((missing + 1)); }

probe_bin() {
    name="$1"
    local_bin="$BIN/$name"
    if [ -x "$local_bin" ]; then
        say_ok "$name" "$local_bin"
        return 0
    fi
    if command -v "$name" >/dev/null 2>&1; then
        say_ok "$name" "$(command -v "$name")"
        return 0
    fi
    say_fail "$name" "not found in $BIN or on PATH"
    return 1
}

echo "we are checking the five core tools"
echo

probe_bin praat        || true
probe_bin mbrola       || true
probe_bin csdr         || true
probe_bin SMILExtract  || true

# Silero-VAD: python glue, not a binary. Require the uv project directory.
if [ -d "$ROOT/python/nadir-vad" ]; then
    say_ok "silero-vad" "$ROOT/python/nadir-vad (uv project)"
else
    say_fail "silero-vad" "python/nadir-vad directory missing"
fi

# Supporting assets: MBROLA voices and openSMILE configs.
if [ -d "$VOICES/us1" ] || [ -d "$VOICES/us3" ]; then
    say_ok "mbrola-voice" "$VOICES"
else
    say_fail "mbrola-voice" "no us1/us3 under $VOICES"
fi

if [ -e "$OPENSMILE_CFG" ]; then
    say_ok "opensmile-cfg" "$OPENSMILE_CFG"
else
    say_fail "opensmile-cfg" "config dir or symlink missing at $OPENSMILE_CFG"
fi

echo
if [ "$missing" -gt 0 ]; then
    printf 'we are missing %d tool(s); see scripts/bootstrap.sh\n' "$missing" >&2
    exit 1
fi
echo "we have all five core tools present"
exit 0
