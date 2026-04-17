#!/usr/bin/env bash
# bootstrap.sh — install the five core tools and sync both toolchains.
# Idempotent. macOS (brew) + Linux (apt) supported.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TOOLS="$ROOT/tools"
BIN="$TOOLS/bin"
SRC="$TOOLS/src"
BUILD="$TOOLS/build"
mkdir -p "$BIN" "$SRC" "$BUILD"

log() { printf "\033[1;34m[bootstrap]\033[0m %s\n" "$*"; }

uname_s="$(uname -s)"
uname_m="$(uname -m)"

# ── Praat ─────────────────────────────────────────────────────────────────────
if ! command -v praat >/dev/null 2>&1; then
    log "installing praat"
    if [[ "$uname_s" == "Darwin" ]]; then
        brew install --cask praat
        # Expose CLI shim
        if [[ ! -x "$BIN/praat" ]]; then
            cat > "$BIN/praat" <<'EOF'
#!/usr/bin/env bash
exec "/Applications/Praat.app/Contents/MacOS/Praat" "$@"
EOF
            chmod +x "$BIN/praat"
        fi
    else
        sudo apt-get update && sudo apt-get install -y praat
    fi
else
    log "praat present: $(praat --version 2>&1 | head -1 || true)"
fi

# ── MBROLA ────────────────────────────────────────────────────────────────────
if ! command -v mbrola >/dev/null 2>&1 && [[ ! -x "$BIN/mbrola" ]]; then
    log "building mbrola"
    if [[ ! -d "$SRC/MBROLA" ]]; then
        git clone --depth 1 https://github.com/numediart/MBROLA "$SRC/MBROLA"
    fi
    (cd "$SRC/MBROLA" && make)
    cp "$SRC/MBROLA/Bin/mbrola" "$BIN/mbrola"
    log "mbrola installed at $BIN/mbrola"
fi

# Voices: pull at least us1 and us3 for English roster.
VOICES_DIR="$TOOLS/mbrola-voices"
mkdir -p "$VOICES_DIR"
if [[ ! -d "$VOICES_DIR/us1" ]]; then
    log "fetching MBROLA voices (us1, us3)"
    if [[ ! -d "$SRC/MBROLA-voices" ]]; then
        git clone --depth 1 https://github.com/numediart/MBROLA-voices "$SRC/MBROLA-voices"
    fi
    for v in us1 us3; do
        if [[ -d "$SRC/MBROLA-voices/data/$v" ]]; then
            cp -r "$SRC/MBROLA-voices/data/$v" "$VOICES_DIR/"
        fi
    done
fi

# ── csdr ──────────────────────────────────────────────────────────────────────
if ! command -v csdr >/dev/null 2>&1 && [[ ! -x "$BIN/csdr" ]]; then
    log "building csdr (jketterl fork)"
    if [[ "$uname_s" == "Darwin" ]]; then
        brew install fftw libsamplerate cmake
    else
        sudo apt-get install -y libfftw3-dev libsamplerate0-dev cmake build-essential
    fi
    if [[ ! -d "$SRC/csdr" ]]; then
        git clone --depth 1 https://github.com/jketterl/csdr "$SRC/csdr"
    fi
    mkdir -p "$BUILD/csdr"
    (cd "$BUILD/csdr" && cmake "$SRC/csdr" -DCMAKE_BUILD_TYPE=Release && make -j)
    cp "$BUILD/csdr/src/csdr-cli/csdr" "$BIN/csdr" 2>/dev/null || \
        cp "$BUILD/csdr/csdr" "$BIN/csdr" 2>/dev/null || true
fi

# ── openSMILE ─────────────────────────────────────────────────────────────────
if ! command -v SMILExtract >/dev/null 2>&1 && [[ ! -x "$BIN/SMILExtract" ]]; then
    log "building openSMILE"
    if [[ ! -d "$SRC/opensmile" ]]; then
        git clone --depth 1 https://github.com/audeering/opensmile "$SRC/opensmile"
    fi
    (cd "$SRC/opensmile" && bash build.sh)
    if [[ -x "$SRC/opensmile/build/progsrc/smilextract/SMILExtract" ]]; then
        cp "$SRC/opensmile/build/progsrc/smilextract/SMILExtract" "$BIN/SMILExtract"
    fi
fi
# Always link configs
if [[ ! -d "$TOOLS/opensmile/config" && -d "$SRC/opensmile/config" ]]; then
    mkdir -p "$TOOLS/opensmile"
    ln -sf "$SRC/opensmile/config" "$TOOLS/opensmile/config"
fi

# ── Rust + Python ─────────────────────────────────────────────────────────────
log "cargo build"
(cd "$ROOT" && cargo build --workspace --release)

log "uv sync (python workspace)"
(cd "$ROOT/python" && uv sync)

# ── Path hint ─────────────────────────────────────────────────────────────────
echo
log "bootstrap done."
log "add to PATH: $BIN"
log "invoke: $ROOT/target/release/nadir doctor"
