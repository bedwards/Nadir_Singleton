#!/usr/bin/env bash
# bootstrap.sh — install the five core tools and sync both toolchains.
#
# Idempotent. macOS (brew) + Linux (apt) supported. Safe to re-run against a
# partially-populated tools/ directory; stages skip when their artefact is
# already in place.
#
# csdr fork selection (see research/csdr.md and tools/VERSIONS.toml):
#   Darwin (all arches)   → ha7ilm/csdr    (command surface matches nadir-dsp)
#   Linux any             → ha7ilm/csdr    (matches the CI verifier)
#   Override: NADIR_CSDR_FORK=jketterl|ha7ilm forces a fork. When the
#   override selects jketterl on Darwin x86_64 the bootstrap auto-applies
#   scripts/patches/csdr-jketterl-macos-x86_64.patch (shm_open/mmap for
#   mremap, sincosf shim, VLA→std::vector in exec.cpp).
#
# Target bash 3.2+ so macOS system bash works. POSIX-ish where possible.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TOOLS="$ROOT/tools"
BIN="$TOOLS/bin"
SRC="$TOOLS/src"
BUILD="$TOOLS/build"
PATCHES="$ROOT/scripts/patches"
mkdir -p "$BIN" "$SRC" "$BUILD"

log()  { printf '\033[1;34m[bootstrap]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[bootstrap]\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31m[bootstrap]\033[0m %s\n' "$*" >&2; exit 1; }

uname_s="$(uname -s)"
uname_m="$(uname -m)"
log "we detect host: ${uname_s}/${uname_m}"

# Number of parallel jobs. Fall back to 2 when neither probe works.
if command -v sysctl >/dev/null 2>&1; then
    JOBS="$(sysctl -n hw.ncpu 2>/dev/null || echo 2)"
elif command -v nproc >/dev/null 2>&1; then
    JOBS="$(nproc 2>/dev/null || echo 2)"
else
    JOBS=2
fi

# Tiny apt wrapper: sudo when non-root, direct when root (e.g. CI containers).
apt_install() {
    if [ "$(id -u)" -eq 0 ]; then
        apt-get update
        apt-get install -y "$@"
    else
        sudo apt-get update
        sudo apt-get install -y "$@"
    fi
}

# ── Praat ─────────────────────────────────────────────────────────────────────
install_praat() {
    if command -v praat >/dev/null 2>&1 || [ -x "$BIN/praat" ]; then
        log "we see praat already; skipping"
        return 0
    fi
    log "we are installing praat"
    if [ "$uname_s" = "Darwin" ]; then
        if ! command -v brew >/dev/null 2>&1; then
            die "we need Homebrew on macOS; install from https://brew.sh first"
        fi
        brew install --cask praat
        # Expose a CLI shim inside tools/bin so PATH activation is sufficient.
        if [ ! -x "$BIN/praat" ]; then
            cat > "$BIN/praat" <<'EOF'
#!/usr/bin/env bash
exec "/Applications/Praat.app/Contents/MacOS/Praat" "$@"
EOF
            chmod +x "$BIN/praat"
        fi
    else
        apt_install praat
    fi
}

# ── MBROLA + voices ───────────────────────────────────────────────────────────
install_mbrola() {
    if command -v mbrola >/dev/null 2>&1 || [ -x "$BIN/mbrola" ]; then
        log "we see mbrola already; skipping build"
    else
        log "we are building mbrola from source"
        if [ "$uname_s" != "Darwin" ]; then
            apt_install build-essential git
        fi
        if [ ! -d "$SRC/MBROLA" ]; then
            git clone --depth 1 https://github.com/numediart/MBROLA "$SRC/MBROLA"
        fi
        (cd "$SRC/MBROLA" && make)
        if [ -x "$SRC/MBROLA/Bin/mbrola" ]; then
            cp "$SRC/MBROLA/Bin/mbrola" "$BIN/mbrola"
        else
            die "we could not find the built mbrola binary under $SRC/MBROLA/Bin"
        fi
        log "mbrola installed at $BIN/mbrola"
    fi

    # Voices: at least us1/us3. Also grab us2 and en1 when present.
    VOICES_DIR="$TOOLS/mbrola-voices"
    mkdir -p "$VOICES_DIR"
    if [ ! -d "$SRC/MBROLA-voices" ]; then
        log "we are fetching the MBROLA-voices repository"
        git clone --depth 1 https://github.com/numediart/MBROLA-voices "$SRC/MBROLA-voices"
    fi
    for v in us1 us3 us2 en1; do
        if [ -d "$VOICES_DIR/$v" ]; then
            continue
        fi
        if [ -d "$SRC/MBROLA-voices/data/$v" ]; then
            cp -r "$SRC/MBROLA-voices/data/$v" "$VOICES_DIR/"
            log "we copied voice $v into $VOICES_DIR/"
        else
            warn "voice $v not found in MBROLA-voices/data; skipping"
        fi
    done
}

# ── csdr ──────────────────────────────────────────────────────────────────────
#
# Fork selection is the key bit of this script. jketterl/csdr is a modern C++17
# multi-call binary with a CMake build; it is the canonical target but it has
# occasionally failed on Apple clang 16 (macOS 26, Darwin 25.x) with errors in
# src/lib/exec.cpp and src/lib/ringbuffer.cpp that boil down to missing
# <cstdint>/<cstring> includes the old clang did not implicitly pull in.
#
# ha7ilm/csdr is the older pure-C Makefile build. It compiles with any system
# compiler and is our stable fallback on Darwin x86_64 and on Linux CI.

select_csdr_fork() {
    if [ -n "${NADIR_CSDR_FORK:-}" ]; then
        printf '%s' "$NADIR_CSDR_FORK"
        return
    fi
    # We default to ha7ilm on every host because nadir-dsp's preset command
    # names (convert_s16_f, fir_interpolate_cc, bandpass_fir_fft_cc, …) come
    # from the ha7ilm CLI surface. The jketterl fork is supported as an opt-in
    # via NADIR_CSDR_FORK=jketterl — on Darwin x86_64 it additionally needs
    # scripts/patches/csdr-jketterl-macos-x86_64.patch to build against Apple
    # clang (no mremap, no portable sincosf, no VLA).
    case "$uname_s/$uname_m" in
        Darwin/*)       printf 'ha7ilm' ;;
        Linux/*)        printf 'ha7ilm' ;;
        *)              printf 'ha7ilm' ;;
    esac
}

install_csdr_deps() {
    if [ "$uname_s" = "Darwin" ]; then
        brew install fftw libsamplerate cmake pkg-config >/dev/null
    else
        apt_install libfftw3-dev libsamplerate0-dev cmake build-essential git pkg-config
    fi
}

build_csdr_jketterl() {
    log "we are building csdr (jketterl fork)"
    if [ ! -d "$SRC/csdr-jketterl" ]; then
        git clone --depth 1 https://github.com/jketterl/csdr "$SRC/csdr-jketterl"
    fi

    # Apply any locally-tracked patches (see scripts/patches/csdr-jketterl-*.patch).
    if [ -d "$PATCHES" ]; then
        for p in "$PATCHES"/csdr-jketterl-*.patch; do
            [ -e "$p" ] || continue
            log "we are applying patch $(basename "$p")"
            (cd "$SRC/csdr-jketterl" && git apply --check "$p" 2>/dev/null \
                && git apply "$p") || warn "patch $p did not apply cleanly; continuing"
        done
    fi

    mkdir -p "$BUILD/csdr-jketterl"
    (cd "$BUILD/csdr-jketterl" \
        && cmake "$SRC/csdr-jketterl" -DCMAKE_BUILD_TYPE=Release \
        && make -j"$JOBS")

    # The canonical binary has lived under a few names/paths across jketterl
    # cuts: src/apps/csdr/csdr (current master), src/csdr-cli/csdr (older),
    # and sometimes the build root. Accept any of them.
    for cand in \
        "$BUILD/csdr-jketterl/src/apps/csdr/csdr" \
        "$BUILD/csdr-jketterl/src/csdr-cli/csdr" \
        "$BUILD/csdr-jketterl/csdr"; do
        if [ -x "$cand" ]; then
            cp "$cand" "$BIN/csdr"
            # Install libcsdr.dylib alongside the binary so a PATH-based
            # launch finds it without extra DYLD_LIBRARY_PATH wiring.
            for lib in \
                "$BUILD/csdr-jketterl/src/lib/libcsdr.dylib" \
                "$BUILD/csdr-jketterl/libcsdr.dylib"; do
                if [ -f "$lib" ]; then
                    cp "$lib" "$BIN/"
                    break
                fi
            done
            log "we installed csdr (jketterl) at $BIN/csdr"
            return 0
        fi
    done
    die "we could not locate a built jketterl csdr binary"
}

build_csdr_ha7ilm() {
    log "we are building csdr (ha7ilm fork)"
    if [ ! -d "$SRC/csdr-ha7ilm" ]; then
        git clone --depth 1 https://github.com/ha7ilm/csdr "$SRC/csdr-ha7ilm"
    fi

    if [ -d "$PATCHES" ]; then
        for p in "$PATCHES"/csdr-ha7ilm-*.patch; do
            [ -e "$p" ] || continue
            log "we are applying patch $(basename "$p")"
            (cd "$SRC/csdr-ha7ilm" && git apply --check "$p" 2>/dev/null \
                && git apply "$p") || warn "patch $p did not apply cleanly; continuing"
        done
    fi

    # ha7ilm ships a plain Makefile that uses GCC-specific flags and Linux
    # syscalls. On Darwin we bypass the Makefile entirely and compile a static
    # binary with clang, stripping the offending flags and defining macOS compat
    # shims for missing Linux constants.
    if [ "$uname_s" = "Darwin" ]; then
        FFTW_CFLAGS="$(pkg-config --cflags fftw3f 2>/dev/null || echo '-I/opt/homebrew/include')"
        FFTW_LIBS="$(pkg-config --libs fftw3f 2>/dev/null || echo '-L/opt/homebrew/lib -lfftw3f')"
        (cd "$SRC/csdr-ha7ilm" && cc -std=gnu99 -O3 -ffast-math \
            -DUSE_FFTW -DLIBCSDR_GPL -DUSE_IMA_ADPCM \
            -D_DARWIN_C_SOURCE \
            -DF_SETPIPE_SZ=0 \
            -DCLOCK_MONOTONIC_RAW=CLOCK_MONOTONIC \
            -Wno-implicit-function-declaration \
            -Wno-unused-result \
            -I. $FFTW_CFLAGS \
            fft_fftw.c libcsdr_wrapper.c csdr.c \
            -lm $FFTW_LIBS -o csdr)
    else
        (cd "$SRC/csdr-ha7ilm" && make)
    fi

    if [ -x "$SRC/csdr-ha7ilm/csdr" ]; then
        cp "$SRC/csdr-ha7ilm/csdr" "$BIN/csdr"
        # Some ha7ilm variants also produce a shared lib the binary dlopens.
        if [ -f "$SRC/csdr-ha7ilm/libcsdr.so" ]; then
            cp "$SRC/csdr-ha7ilm/libcsdr.so" "$BIN/"
        fi
        log "we installed csdr (ha7ilm) at $BIN/csdr"
        return 0
    fi
    die "we could not locate a built ha7ilm csdr binary"
}

install_csdr() {
    if [ -x "$BIN/csdr" ] || command -v csdr >/dev/null 2>&1; then
        log "we see csdr already; skipping"
        return 0
    fi
    install_csdr_deps
    fork="$(select_csdr_fork)"
    log "we chose csdr fork: $fork"
    case "$fork" in
        jketterl) build_csdr_jketterl ;;
        ha7ilm)   build_csdr_ha7ilm ;;
        *)        die "unknown csdr fork: $fork (set NADIR_CSDR_FORK)" ;;
    esac
}

# ── openSMILE ─────────────────────────────────────────────────────────────────
install_opensmile() {
    if [ -x "$BIN/SMILExtract" ] || command -v SMILExtract >/dev/null 2>&1; then
        log "we see SMILExtract already; skipping build"
    else
        log "we are building openSMILE"
        if [ "$uname_s" = "Darwin" ]; then
            brew install cmake >/dev/null
        else
            apt_install cmake build-essential git
        fi
        if [ ! -d "$SRC/opensmile" ]; then
            git clone --depth 1 https://github.com/audeering/opensmile "$SRC/opensmile"
        fi
        (cd "$SRC/opensmile" && bash build.sh)
        for cand in \
            "$SRC/opensmile/build/progsrc/smilextract/SMILExtract" \
            "$SRC/opensmile/bin/SMILExtract"; do
            if [ -x "$cand" ]; then
                cp "$cand" "$BIN/SMILExtract"
                break
            fi
        done
        if [ ! -x "$BIN/SMILExtract" ]; then
            die "we could not locate a built SMILExtract binary"
        fi
    fi

    # Always (re)link configs so graph recipes keep resolving.
    if [ ! -e "$TOOLS/opensmile/config" ] && [ -d "$SRC/opensmile/config" ]; then
        mkdir -p "$TOOLS/opensmile"
        ln -sf "$SRC/opensmile/config" "$TOOLS/opensmile/config"
        log "we linked opensmile config at $TOOLS/opensmile/config"
    fi
}

# ── Rust + Python ─────────────────────────────────────────────────────────────
build_rust_workspace() {
    if ! command -v cargo >/dev/null 2>&1; then
        die "we need cargo on PATH; install rustup from https://rustup.rs"
    fi
    log "we are running cargo build --workspace --release"
    (cd "$ROOT" && cargo build --workspace --release)
}

sync_python() {
    if ! command -v uv >/dev/null 2>&1; then
        die "we need uv on PATH; install from https://docs.astral.sh/uv/"
    fi
    log "we are running uv sync in python/"
    (cd "$ROOT/python" && uv sync)
}

# ── orchestration ─────────────────────────────────────────────────────────────
install_praat
install_mbrola
install_csdr
install_opensmile
build_rust_workspace
sync_python

echo
log "bootstrap done; we are running 'nadir doctor' to confirm the five tools"
echo
PATH="$BIN:$PATH" "$ROOT/target/release/nadir" doctor || \
    warn "nadir doctor reported issues; see output above"

echo
log "tip: add to PATH → $BIN"
