# csdr — Research note (April 2026)

## What it is

`csdr` is a streaming digital signal processing (DSP) library and command-line
tool originally written by Andras Retzler (HA7ILM) as the DSP backbone of the
OpenWebRX web-SDR receiver. Its contract is deliberately UNIX: each DSP block
is either a standalone binary (in the original `ha7ilm/csdr`) or a subcommand
of a single multi-call binary (`csdr <function>` in the modernized
`jketterl/csdr`). Blocks read raw binary samples from `stdin`, write raw
binary samples to `stdout`, and compose through shell pipes. There is no
plugin API, no graph scheduler, no sample-rate metadata on the wire — the
runtime model is "let the kernel move bytes between processes."

Two active forks matter in April 2026:

- **`ha7ilm/csdr`** — Retzler's original. Pure C, Makefile build, dense
  manually-vectorized inner loops. Effectively frozen around 2020.
- **`jketterl/csdr`** — Jakob Ketterl's modernized fork used by the
  currently-maintained OpenWebRX+ line. C++ with a CMake build, split into a
  shared library (`libcsdr`) and a thin multi-call CLI (`csdr`). Depends on
  `libfftw3` and `libsamplerate`. Adds real resampling, FIFO-based runtime
  parameter injection, and cleaner error handling. No tagged GitHub
  releases — we pin by commit SHA. As of April 2026 the `master` branch is
  the stable cut shipped in the OpenWebRX+ package.
- A luarvique/csdr fork also exists on top of jketterl but we do not target it.

**For Nadir\_Singleton we target `jketterl/csdr` where it builds, and fall
back to `ha7ilm/csdr` where it does not.** The C++/CMake build survives
clang on macOS arm64 without the NEON-intrinsics surgery ha7ilm's Makefile
demands, libsamplerate gives us high-quality arbitrary-ratio resampling
between our 48 kHz master and MBROLA's 16 kHz, and the runtime FIFO
protocol matches our need to modulate effect params from `nadir-dsp`. The
command surface in both forks is nearly identical — if we later need a
ha7ilm-only block we can reach for its binary.

### Per-platform fork choice (April 2026)

| Platform          | Fork         | Rationale                                                         |
|-------------------|--------------|-------------------------------------------------------------------|
| Darwin arm64      | `jketterl`   | Apple-clang builds cleanly; libsamplerate + FIFO protocol.        |
| Darwin x86_64     | `ha7ilm`     | macOS 26's Apple clang 16 fails to build jketterl src/lib C++.    |
| Linux (CI)        | `ha7ilm`     | Matches what our verify workflow expects; no libsamplerate path.  |

The jketterl build failure we observed on macOS 26 / x86\_64 manifested as
four C++ compile errors in `src/lib/CMakeFiles/csdr.dir/exec.cpp.o` and
`ringbuffer.cpp.o` — the typical "missing `<cstdint>` / `<cstring>`
includes" regression that appears when Apple clang tightens its implicit
header set. Rather than carry a drifting patch against upstream master we
switch forks on that host. `scripts/bootstrap.sh` selects automatically
based on `uname -s` / `uname -m`; override with
`NADIR_CSDR_FORK=jketterl|ha7ilm`. If a downstream recipe relies on a
jketterl-only block (`bandpass_fir_fft_cc --fifo`, `fastagc_ff` flag
parsing), `nadir-dsp` flags it at graph-validate time.

When the jketterl fork is the chosen target, `scripts/bootstrap.sh` also
applies any patches under `scripts/patches/csdr-jketterl-*.patch` before
configure. Reserve that directory for genuinely needed source fixes, not
feature adds.

## Installation

On macOS arm64 (Apple Silicon), build from source:

```bash
brew install fftw libsamplerate cmake pkg-config
git clone https://github.com/jketterl/csdr.git
cd csdr
git checkout master        # or pin to a reviewed SHA
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(sysctl -n hw.ncpu)
sudo make install
```

The install lays down `libcsdr.dylib` under `/usr/local/lib` and a
single multi-call binary `csdr` under `/usr/local/bin`. Invocation is
always:

```bash
csdr <function_name> [arg1 arg2 ...]
```

Verify the install:

```bash
csdr ?agc          # lists any function name matching "agc"
csdr =1/0.0        # evaluates a Python expression; useful for rate math
```

Nadir\_Singleton's build script under `tools/vendor/build_csdr.sh` pins the
jketterl commit, builds it under `tools/vendor/csdr/build`, and symlinks the
binary into `tools/bin/csdr` so the repo is hermetic against whatever
Homebrew the user happens to have.

## Sample-format conventions

csdr streams are **raw binary**, little-endian, host byte order, no header,
no metadata, no rate tagging. The function-name suffix tells you the wire
format; Nadir\_Singleton conventions follow.

| Suffix | Wire format                                       | Our use                                    |
|--------|---------------------------------------------------|--------------------------------------------|
| `_f`   | IEEE 754 float32, interleaved if a vector         | Internal audio bus                         |
| `_c`   | Complex float32 (I, Q pairs: 2 × f32 per sample)  | Analytic signal for shifts / modulation     |
| `_s16` | Signed 16-bit little-endian PCM                   | Interop with WAV / MBROLA                  |
| `_u8`  | Unsigned 8-bit                                    | Symbol streams, packed bits                |
| `_s8`  | Signed 8-bit                                      | Rare                                        |
| `_s24` | Signed 24-bit packed (supports `--bigendian`)     | High-bit-depth masters                     |

Suffix letters combine: `_cf` = complex in, float out; `_ff` = real float in,
real float out; `_cc` = complex in, complex out.

Key format-conversion functions:

- `convert_s16_f` — 16-bit PCM into our float bus. Implicit `*(1.0/32768.0)`
  scaling.
- `convert_f_s16` — float back out to PCM for writing WAV / feeding a
  player.
- `convert_u8_f` — rtl\_sdr-style 8-bit centred at 127.5 into float; we use
  this only when pulling in a legacy sample.
- `convert_s24_f [--bigendian]` / `convert_f_s24` — high-bit-depth path for
  masters.
- `realpart_cf` — discard the imaginary part of a complex stream to drop
  back to a real audio bus after a shift / ring-mod.
- `dump_f` / `dump_u8` — ASCII debug dumps (floats as `%g`, bytes as hex).

**Endianness** is always little-endian for single- and two-byte formats,
matching arm64 / x86-64 native order. A WAV written on a PPC would need
byte-swapping before `convert_s16_f`, but that is not a concern today.

## Block categories

The full function surface in the jketterl fork at the pinned April-2026 commit
is roughly 90 operators. Grouping them as our graph DSL will:

**Conversions**

- `convert_u8_f`, `convert_f_u8`
- `convert_s8_f`, `convert_f_s8`
- `convert_s16_f`, `convert_f_s16`
- `convert_s24_f [--bigendian]`, `convert_f_s24 [--bigendian]`
- `realpart_cf` — complex to real
- `dsb_fc [q_value]` — real to complex DSB envelope (see Modulation)

**Resampling**

- `fir_decimate_cc <decimation_factor> [transition_bw [window]]` — complex
  decimating FIR, `window` one of `HAMMING` / `BLACKMAN` / `BOXCAR`.
- `fir_interpolate_cc <interpolation_factor> [transition_bw [window]]`
- `fractional_decimator_ff <rate> [num_poly_points [transition_bw [window]] | --prefilter]`
- `fractional_decimator_cc <rate> [ ... | --prefilter]`
- `rational_resampler_ff <interpolation> <decimation> [transition_bw [window]]`
- `plain_interpolate_cc <interpolation>` — zero-insertion (no filter)

**Filters**

- `firdes_lowpass_f <cutoff_rate> <length> [window [--octave]]` — emits
  tap coefficients to stdout.
- `firdes_bandpass_c <low_cut> <high_cut> <length> [window [--octave]]`
- `firdes_peak_c <rate> <length> [window [--octave]]`
- `bandpass_fir_fft_cc <low_cut> <high_cut> <transition_bw> [window]` —
  overlap-add FFT bandpass. Cutoffs are normalized (`rate / fs`, range
  -0.5..0.5). Supports `--fifo <path>` for live retuning.
- `peaks_fir_cc <taps_length> <peak_rate_1> ... <peak_rate_N>` — comb of
  narrow peaks.
- `pulse_shaping_filter_cc (RRC <sps> <num_taps> <beta> | COSINE <sps>)`
- `dcblock_ff`, `fastdcblock_ff`

**FFT / Spectral**

- `fft_cc <fft_size> <out_of_every_n_samples> [window [--octave] [--benchmark]]`
- `fft_fc <fft_out_size> <out_of_every_n_samples> [--benchmark]`
- `logpower_cf [add_db]`
- `logaveragepower_cf <add_db> <fft_size> <avgnumber>`
- `fft_exchange_sides_ff <fft_size>`, `fft_one_side_ff <fft_size>`

**Modulation / Demodulation**

- `fmdemod_quadri_cf`, `fmdemod_atan_cf`, `fmdemod_quadri_novect_cf`
- `amdemod_cf`, `amdemod_estimator_cf`
- `fmmod_fc`
- `dsb_fc [q_value]` — real-to-complex double-sideband generator; in the
  music context this is the cleanest way to build an envelope that can be
  shifted in frequency. The original ha7ilm README also lists `ssb_cf` in
  some builds; the jketterl fork favours building SSB from
  `bandpass_fir_fft_cc` + `realpart_cf`.
- `deemphasis_wfm_ff <sample_rate> <tau>` — single-pole IIR lowpass in the
  style of broadcast FM de-emphasis. We repurpose this as a *treble
  softener* on noise beds.
- `deemphasis_nfm_ff <predefined_sample_rate>`

**Frequency shift** (`rate` is `hz / fs`, range -0.5..0.5)

- `shift_math_cc <rate>` — canonical sin/cos reference.
- `shift_addition_cc <rate>` — trig-addition recurrence (~4× faster),
  supports `--fifo` for live retuning.
- `shift_table_cc <rate> [table_size]`, `shift_addfast_cc`, `shift_unroll_cc` — fast paths.
- `shift_addition_fc <rate>` — real-to-complex + shift in one go.
- `decimating_shift_addition_cc <rate> [decimation]`

**Arithmetic**

- `gain_ff <gain>` — scalar multiply.
- `add_const_cc <i> <q>` — complex bias.
- `add_n_zero_samples_at_beginning_f <n>`
- `yes_f <value> [buf_times]` — constant float generator (useful as a
  multiplier source).
- Parameter-piping via `--fifo` on `gain_ff` (jketterl extension) gives us
  time-varying multiplication from a control process.

**Control / Dynamics**

- `agc_ff [--profile (slow|fast)] [--hangtime T] [--reference R] [--attack A] [--decay D] [--max M] [--initial I] [--attackwait W] [--alpha L]` (jketterl fork uses flags; ha7ilm uses positional args).
- `agc_s16 [same options]`, `fastagc_ff [block_size [ref]]`, `simple_agc_cc <rate> [ref [max_gain]]`.
- `limit_ff [max_amplitude]` — hard clipper, default `±1.0`.
- `fixed_amplitude_cc <new_amplitude>` — phase-preserving magnitude set.
- `squelch_and_smeter_cc --fifo <sq> --outfifo <sm> <use_nth> <report_nth>`
- `clipdetect_ff`, `detect_nan_ff`

**Noise & generators**

- `uniform_noise_f` — white-ish uniform noise in `[-1, 1]`.
- `gaussian_noise_c` — Gaussian complex noise.
- `awgn_cc <snr_db> [--snrshow]`
- `yes_f <value> [buf_times]`

**Stream plumbing**

- `clone`, `through` (same plus prints data rate), `none`, `tee <path>`,
  `fifo <buffer_size> <num_buffers>`, `flowcontrol <rate> <reads_per_sec>`,
  `setbuf <buffer_size>`, `dump_f`, `dump_u8`.

**Audio (stereo helpers)**

- `mono2stereo_s16`, `stereo2mono_s16` — only work in `_s16` space; convert
  before/after float processing.

**Meta**

- `csdr ?<term>` — grep function names.
- `csdr =<python_expr>` — evaluate Python for rate math inline.

(The ha7ilm README lists a few variants — `ssb_cf`, `old_fractional_decimator_ff`,
some NEON `*_addfast_*` shifts — that are either absent or renamed in the
jketterl fork; `nadir-dsp` must validate against the binary we actually
shipped, not the README. We do that by running `csdr ?` at build time and
snapshotting the set into `crates/nadir-dsp/src/csdr_ops.rs`.)

## Musical repurposing

csdr is built for SDR, but every block is a mathematical operator on a
stream of floats. The mapping to musical effects is mostly a question of
choosing the right block plus the right rate.

- **Reverb-ish / chorus-ish smear.** Cascade
  `fir_interpolate_cc <N>` followed by `fir_decimate_cc <N>` where both
  specify aggressive `transition_bw` (e.g. `0.05`). The group delay of the
  symmetric FIR plus the slight ringing creates a short comb-like tail.
  Run the dry signal through `tee` and `fifo` into a parallel branch,
  merge with `gain_ff 0.3`. This is not a real reverb but a cheap
  short-room colour.
- **Ring mod.** Convert real audio to complex with `shift_addition_fc 0`
  (or `dsb_fc`), then `shift_addition_cc <rate>` where `rate = carrier_hz
  / sample_rate`, then `realpart_cf`. A 40 Hz carrier at 48 kHz is
  `rate = 40/48000 = 0.000833333`. The FIFO hook on
  `shift_addition_cc --fifo` lets `nadir-dsp` sweep the carrier.
- **Sidechain duck.** Build a voice-activity probability stream (from
  `nadir-vad` or an RMS follower) as raw `f32`, then drive `gain_ff --fifo`
  with the inverse envelope. Because `gain_ff` reads the fifo
  line-by-line we buffer control updates to one write per ~1 ms for a
  smooth duck; heavier smoothing stays in the controller.
- **Granular texture.** Use `dsb_fc` to produce an envelope-modulated
  complex stream, then bandpass with `bandpass_fir_fft_cc` around a
  chosen formant, then `realpart_cf`. Chop by alternating `gain_ff 0` /
  `gain_ff 1` via fifo on a 20–80 ms window. With tens of such branches
  mixed, you get granular smears.
- **Pitch-shift approximation.** True phase-vocoder is out of scope for
  csdr, but naïve tape-style pitch shift is:
  `fir_interpolate_cc 3 | fir_decimate_cc 2` gives a ×1.5 rate change
  (and therefore pitch shift) while length-scaling the signal; pair with
  an external time-stretch (`rubberband`) to restore duration. Good
  enough for Nadir\_Singleton's "detuned choir" pad.
- **Formant shaper.** `peaks_fir_cc` with two or three peak rates
  corresponding to formants of a target vowel builds a resonant mask we
  apply to pink noise to turn it into a whispered vowel bed.

All of these live as named recipes under
`research/csdr_recipes/` once authored.

## Pipeline syntax examples

These three are ready-to-paste demonstrations. Sample rate is 48 kHz
throughout unless a stage forces otherwise.

**1. Band-limit a mono vocal and ring-modulate with a 40 Hz carrier.**
Input is 48 kHz mono `s16` WAV read from `vocal.wav`; output is
48 kHz mono `s16` to `out.wav`.

```bash
# rate = 40 / 48000 = 0.0008333333
sox vocal.wav -t raw -r 48000 -c 1 -b 16 -e signed-integer - \
  | csdr convert_s16_f \
  | csdr shift_addition_fc 0.0 \
  | csdr bandpass_fir_fft_cc 0.002 0.08 0.005 HAMMING \
  | csdr shift_addition_cc 0.0008333333 \
  | csdr realpart_cf \
  | csdr limit_ff 0.98 \
  | csdr convert_f_s16 \
  | sox -t raw -r 48000 -c 1 -b 16 -e signed-integer - out.wav
```

`shift_addition_fc 0.0` turns real into a minimal analytic complex stream
without frequency translation, the bandpass restricts the vocal to 96 Hz –
3840 Hz, the second shift applies the ring-mod carrier, `realpart_cf`
drops back to real.

**2. Generate a shaped "pink-ish" noise bed with FIR shaping.**
Uniform white noise is spectrally flat; we shape it by cascaded lowpass
FIRs to get a rolloff that approximates pink (-3 dB/oct). Output is a
10-second 48 kHz mono `s16` WAV.

```bash
# 10 s × 48000 samp × 4 B/f32 = 1920000 bytes
csdr yes_f 0 1 </dev/null >/dev/null   # sanity check the binary exists
(
  csdr uniform_noise_f \
    | csdr gain_ff 0.3 \
    | csdr dcblock_ff \
    | csdr shift_addition_fc 0.0 \
    | csdr bandpass_fir_fft_cc -0.45 0.45 0.02 HAMMING \
    | csdr realpart_cf \
    | csdr deemphasis_wfm_ff 48000 500e-6 \
    | csdr limit_ff 0.9 \
    | csdr convert_f_s16 \
    | head -c 1920000
) | sox -t raw -r 48000 -c 1 -b 16 -e signed-integer - noise_bed.wav
```

`deemphasis_wfm_ff` is our trick: its single-pole IIR is exactly the
shape we want for a warm-ish noise colour. Tuning `tau` trades brightness
against thump.

**3. Mix two stems via named-pipe fanout.**
We want to hear `drums.raw` and `pad.raw` (both 48 kHz mono float32)
summed with independent gains, without first writing a combined file.

```bash
mkfifo /tmp/nadir_drums /tmp/nadir_pad

# Feed each source into its fifo in the background.
cat drums.raw | csdr gain_ff 0.8 > /tmp/nadir_drums &
cat pad.raw   | csdr gain_ff 0.4 > /tmp/nadir_pad   &

# Paste-merge the two float streams sample-wise and sum.
# csdr has no native mixer, so we use awk on interleaved floats via od.
paste -d '' /tmp/nadir_drums /tmp/nadir_pad \
  | csdr =0   # placeholder — see note below
```

csdr does not ship a two-input mixer out of the box — its one-process-one-
pipeline philosophy assumes a single input stream. For real mixing we
either (a) use `sox -m` outside the pipeline, (b) drop the mix into a
small Rust helper (`nadir-dsp`'s `mix` op, which reads two fifos of
`f32` and writes the sum), or (c) use `pcm.mix` from `ffmpeg -filter_complex
amix`. Our convention: **mix outside csdr, filter inside csdr.**

## Sample-rate strategy

csdr does not carry sample-rate in the stream. It is a frame-by-frame
operator, and any block that needs a rate (`deemphasis_wfm_ff`,
`fractional_decimator_ff`, `fir_decimate_cc` when you care about
transition-bw in Hz) takes the rate as a CLI argument or expects you to
normalize frequencies against it (`shift_addition_cc` uses
`rate = hz / fs`). Consequences:

- If we mis-type a rate the DSP silently produces wrong-sounding audio
  rather than erroring.
- Re-ordering stages or splicing two graphs from different rates is a
  common source of bugs.

**Nadir\_Singleton convention.**

- Audio master rate: **48 kHz**.
- MBROLA vocal synth outputs **16 kHz** — pipelines that accept MBROLA
  output must either upsample first (`rational_resampler_ff 3 1 0.005
  HAMMING`) or run at 16 kHz and resample at the end.
- Every graph TOML (see the DSL section) declares `in_sr` and `out_sr` at
  the top, and `nadir-dsp` validates that every per-stage rate argument
  is consistent with `in_sr` up to that stage.
- Upsampling always goes through `rational_resampler_ff` (libsamplerate
  quality) for correctness, never `plain_interpolate_cc`.

The rate block in every graph TOML looks like:

```toml
# Rate contract — every csdr graph must declare these.
in_sr  = 48000
out_sr = 48000
```

## Driving from Rust

`nadir-dsp` spawns csdr pipelines with `std::process::Command`. The pattern
is one `Command` per stage, chained by wiring each child's `stdout` into
the next child's `stdin`.

```rust
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;

/// Build a three-stage csdr pipeline:
///     convert_s16_f | bandpass_fir_fft_cc 0.01 0.4 0.005 | convert_f_s16
pub fn ring_bandpass(input_s16: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut stage1 = Command::new("csdr")
        .args(["convert_s16_f"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stage2 = Command::new("csdr")
        .args(["bandpass_fir_fft_cc", "0.01", "0.4", "0.005", "HAMMING"])
        .stdin(Stdio::from(stage1.stdout.take().unwrap()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stage3 = Command::new("csdr")
        .args(["convert_f_s16"])
        .stdin(Stdio::from(stage2.stdout.take().unwrap()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Writer thread: feed input into stage1 and close.
    let mut in_handle = stage1.stdin.take().unwrap();
    let input = input_s16.to_vec();
    let writer = thread::spawn(move || -> std::io::Result<()> {
        in_handle.write_all(&input)?;
        Ok(())
    });

    // Stderr drain threads — critical for avoiding deadlock.
    for (name, stderr) in [
        ("stage1", stage1.stderr.take()),
        ("stage2", stage2.stderr.take()),
        ("stage3", stage3.stderr.take()),
    ] {
        if let Some(mut e) = stderr {
            thread::spawn(move || {
                let mut buf = Vec::new();
                let _ = e.read_to_end(&mut buf);
                if !buf.is_empty() {
                    tracing::debug!(target: "csdr", stage = name,
                        "{}", String::from_utf8_lossy(&buf));
                }
            });
        }
    }

    // Collect stage3's stdout on the main thread.
    let mut out = Vec::new();
    stage3.stdout.take().unwrap().read_to_end(&mut out)?;

    writer.join().unwrap()?;
    stage1.wait()?;
    stage2.wait()?;
    stage3.wait()?;
    Ok(out)
}
```

**Deadlock avoidance notes.**

- csdr processes write progress and warnings to `stderr`. A full pipe on
  `stderr` blocks the process. We always drain `stderr` on a dedicated
  thread (or `Stdio::null()` if we genuinely don't care).
- The input writer must run on its own thread when the output reader is
  also on the main thread; otherwise they can both block each other
  through the kernel pipe buffer (64 KiB on macOS).
- `stage1.stdout.take()` transfers ownership into stage2's stdin; do not
  hold a clone in the parent. The parent must not read stage2's stdout
  either — only the last stage's stdout is parent-visible.
- Parameter FIFOs (for `shift_addition_cc --fifo /tmp/foo`) are the same
  pattern: create with `nix::unistd::mkfifo`, keep the write-side handle
  on a control thread that emits newline-terminated floats.
- Backpressure is real. A downstream stage reading slowly will stall the
  entire chain; the upstream stage blocks in `write(2)` once its pipe
  is full. This is the right behaviour — we want source rate to match
  sink rate. For offline rendering it is a non-issue.

## Graph DSL

Nadir\_Singleton expresses a csdr pipeline as a TOML document. The TOML is
validated by `nadir-dsp`, which materializes it into a Rust `Vec<Command>`
equivalent to the snippet above.

Schema (core):

```toml
# Rate contract.
in_sr  = 48000
out_sr = 48000

# Optional graph-level metadata
name        = "vocal_ringmod_40"
description = "Band-limit a vocal and ring-modulate with a 40 Hz carrier."

# Ordered list of stages; each maps to one `csdr <cmd> <args>` child.
[[stage]]
cmd  = "convert_s16_f"

[[stage]]
cmd  = "shift_addition_fc"
args = ["0.0"]

[[stage]]
cmd  = "bandpass_fir_fft_cc"
args = ["0.01", "0.4", "0.005", "HAMMING"]

[[stage]]
cmd  = "shift_addition_cc"
args = ["0.0008333333"]
# Runtime-tunable parameters bind a named control port to a FIFO.
fifo = { name = "carrier_rate", path = "/tmp/nadir_{graph}_carrier" }

[[stage]]
cmd  = "realpart_cf"

[[stage]]
cmd  = "limit_ff"
args = ["0.98"]

[[stage]]
cmd  = "convert_f_s16"
```

Validation rules `nadir-dsp` enforces:

1. `cmd` must be in the known-op set snapshotted at build time from
   `csdr ?`.
2. `args` arity and types are checked against a per-op schema file.
3. The wire-format suffix chain must be consistent: stage N's output
   type must match stage N+1's input type, with explicit
   `convert_*` stages wherever a type crosses (`_s16` → `_f` → `_c` →
   `_f` → `_s16`). This catches the most common class of csdr bugs.
4. Every stage that takes a rate in Hz (`deemphasis_wfm_ff`,
   `rational_resampler_ff` ratios) is validated against `in_sr` /
   `out_sr` plus any upstream decimation or interpolation — the graph's
   per-point rate is tracked symbolically.
5. FIFO paths are expanded with `{graph}` = a hash of the graph name so
   parallel renders do not collide.

Every track's `render.lock.toml` embeds the fully-expanded csdr pipeline
string (the exact shell form we would pipe) alongside the csdr binary
SHA256, so renders are reproducible.

## Known pitfalls

- **Endianness.** All multi-byte formats are host little-endian. Big-endian
  captures need an explicit byte-swap before `convert_s16_f`.
- **Buffer sizing.** Static defaults are 1024 samples narrowband, 16384
  wideband. For 48 kHz mono `f32` the 1024 default is ~21 ms latency per
  stage. Tune with `CSDR_FIXED_BUFSIZE=<n>` or
  `CSDR_DYNAMIC_BUFSIZE_ON=1`; we set fixed sizes in the render env.
- **No in-stream metadata.** No rate, channel, timestamp, or flush. Splicing
  streams of different rates yields a pitch-shifted mess. The TOML
  validates rates statically.
- **Channel count.** csdr is mono except for `mono2stereo_s16` /
  `stereo2mono_s16`. Stereo = two parallel pipelines joined outside csdr.
- **Fork drift.** `agc_ff` flag handling differs between ha7ilm (positional)
  and jketterl (`--option value`). The `nadir-dsp` op schema is authoritative.
- **`ssb_cf` is absent in jketterl.** Rewrite as `bandpass_fir_fft_cc` on a
  shifted complex stream plus `realpart_cf`.
- **Silent death.** If stage N dies, stage N+1 sees EOF and exits clean.
  The orchestrator must inspect exit codes and stderr.
- **No tagged releases on jketterl.** Pin a commit SHA, rebuild in CI.
- **macOS arm64.** ha7ilm's NEON-targeted Makefile fails on Apple Silicon;
  jketterl's CMake path side-steps this with clean C++ and auto-vectorization.
- **`csdr =<expr>`** imports `os` and `sys` — never feed it untrusted input.
- **Pipe buffer is 64 KiB on macOS** (~0.3 s of 48 kHz f32 mono). A stage
  that blocks longer than that stalls upstream.

## How we use it in Nadir\_Singleton

csdr is the DSP backbone for **all non-vocal audio** in Nadir\_Singleton.
Drums, pads, textures, impact hits, glue on the master bus — every effect
chain is ultimately a csdr pipeline plus at most a thin Rust or `sox`
bookend for format and mixing.

- `crates/nadir-dsp` owns the graph DSL, schema validation, child-process
  orchestration, FIFO control plane, and logging. It is the only code path
  that invokes `csdr` directly.
- Every effect preset under `albums/<album>/fx/*.toml` is a graph in the DSL
  above, version-controlled alongside the song.
- Every track render emits `render.lock.toml` listing the exact csdr
  pipeline strings, args, environment (buffer sizes, rate), and the SHA256
  of the `csdr` binary used — the reproducibility contract.
- Vocals: MBROLA → sox (16→48 kHz) → csdr (EQ, comp, fx) → mix bus. The
  rate conversion lives in sox because its quality is well-studied.
- Live parameter automation (LFOs, envelope followers, sidechain) drives
  csdr via `--fifo` control ports. `nadir-dsp` exposes a `ControlPort`
  abstraction that writes newline-separated floats.
- CI runs every graph under the schema validator, renders a 1-s reference,
  and hashes the output. Any csdr upgrade must keep all hashes stable.

Custom Rust DSP stages expose the same raw-f32 stdin-to-stdout contract and
appear in the TOML as e.g. `cmd = "nadir_pitch_shift"`, indistinguishable
from a csdr block.

## References

- ha7ilm/csdr (original): <https://github.com/ha7ilm/csdr>
- jketterl/csdr (fork we target): <https://github.com/jketterl/csdr>
- luarvique/csdr (third-party fork): <https://github.com/luarvique/csdr>
- OpenWebRX+ project: <https://github.com/jketterl/openwebrx>
- OpenWebRX+ manual install wiki:
  <https://github.com/jketterl/openwebrx/wiki/Manual-Package-installation-(including-digital-voice)>
- owrx\_connector (pins csdr >= 0.18):
  <https://github.com/jketterl/owrx_connector>
- Andras Retzler, "Simple DSP algorithms for software-defined radio",
  BME thesis, Budapest University of Technology and Economics, 2014.
- ARRL Handbook, chapter on SDR DSP — background for `fmdemod_quadri_cf`,
  `amdemod_cf`, and `agc_ff`.
- libfftw3: <http://www.fftw.org/>
- libsamplerate: <http://libsndfile.github.io/libsamplerate/>
- Internal: `crates/nadir-dsp/src/csdr_ops.rs`,
  `tools/vendor/build_csdr.sh`, `albums/*/fx/*.toml`.
