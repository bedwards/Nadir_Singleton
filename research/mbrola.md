# MBROLA — Research note (April 2026)

## What it is

MBROLA is a diphone-concatenative speech synthesizer. A "diphone" is the
audio spanning the latter half of one phoneme through the first half of
the next, capturing the coarticulatory transition that is the hardest
part of a phoneme to synthesize from scratch. A small database
(~1500–2500 pre-recorded units per speaker) is pitch- and phase-normalized
at database-creation time so that arbitrary phoneme sequences can be
concatenated and reprosodized at synthesis time via a time-domain
PSOLA-like algorithm.

MBROLA was developed beginning in 1995 at the TCTS Lab of the Faculté
Polytechnique de Mons (Belgium), primarily by Thierry Dutoit with Vincent
Pagel, N. Pierret, F. Bataille and O. Van der Vrecken. Seminal reference:
Dutoit et al., "The MBROLA project: towards a set of high quality speech
synthesizers free of use for non commercial purposes," ICSLP 1996,
pp. 1393–1396 (DOI 10.21437/ICSLP.1996-356).

For over twenty years MBROLA was distributed as closed binaries under a
"free for non-commercial use" clause. On 24 October 2018 the engine
source was published on GitHub under the GNU AGPL v3 by the numediart
institute (the successor research group to the original TCTS lab). The
companion MBROLATOR database-creation tool followed on 23 January 2019
under the same license. The engine itself is AGPL; individual voice
databases ship their own licenses inherited from donor speakers and
institutions — most remain "free for non-commercial use only," with
direct consequences for Nadir_Singleton release policy (see "Known
pitfalls").

MBROLA is phoneme-to-speech, not text-to-speech. It does not perform
grapheme-to-phoneme conversion, stress assignment, or intonation
prediction. You hand it a `.pho` file and it hands back raw 16-bit
linear PCM at the database's native sample rate. Everything linguistic
upstream of the `.pho` file is the caller's problem — exactly what we
want for text-to-sung-audio where we need full control of every
phoneme's pitch and timing.

## Installation

MBROLA is a single C program with no external dependencies beyond libc
and a Makefile build.

```bash
git clone https://github.com/numediart/MBROLA.git
cd MBROLA
make
sudo cp Bin/mbrola /usr/local/bin/mbrola
```

There is no autoconf, CMake, or pkg-config. `make clean` wipes the
build.

macOS specifics (including Apple Silicon / arm64) require minor patches
because the source predates modern Unix headers:

1. `Misc/common.h` declares its own `swab()` prototype with `int nbytes`,
   colliding with Darwin's `<unistd.h>` which uses `ssize_t`. See
   https://github.com/numediart/MBROLA/issues/9. Fix: delete the local
   prototype or guard it with `#ifndef __APPLE__`.
2. The Makefile has a commented-out `CFLAGS += -DTARGET_OS_MAC` line.
   Enable it on macOS. arm64 is little-endian like x86_64, so the same
   flag works for both Intel and Apple Silicon.
3. arm64 otherwise builds unchanged — pure scalar C, no SIMD, no
   NEON/SSE portability concerns.
4. The `mbrowrap` Python glue used by some downstream projects assumes
   `/proc` and will not work on macOS, but we invoke the binary directly
   from Rust.

Voice data is installed separately. Conventional layout:

```
/usr/local/share/mbrola/us1/us1
/usr/local/share/mbrola/us1/README.txt
/usr/local/share/mbrola/us1/license.txt
/usr/local/share/mbrola/en1/en1
...
```

Each voice directory contains a single binary (~2–6 MB) whose filename
matches the voice code with no extension — that is the diphone database.
There are no `.mbrola` extension files; the voicedb argument on the
command line is either the directory containing the binary or the
binary itself.

## Voice databases

All voices live at https://github.com/numediart/MBROLA-voices (the
`data/` subtree). Approximately 80+ voices cover 40+ languages, named
`<language-code><digit>` where the digit distinguishes speaker or
dialect.

Partial catalog:

- **English (American):** `us1` (female, ~180 Hz ref pitch), `us2`
  (male, ~115 Hz), `us3` (male, ~125 Hz). All 16 kHz.
- **English (British):** `en1` (female, RP). 16 kHz.
- **French:** `fr1`–`fr7` (mixed male/female, Parisian + Belgian).
- **German:** `de1`–`de8` (`de8` Bavarian). `de5`/`de6` are 22.05 kHz;
  rest 16 kHz.
- **Spanish:** `es1`–`es4`, plus `mx1`, `mx2`, `vz1` (Latin American).
- **Italian:** `it1`–`it4`.
- **Portuguese:** `br1`–`br4` (Brazilian), `pt1` (European).
- **Other European:** Dutch `nl1`–`nl3`, Polish `pl1`, Czech `cz1`/`cz2`,
  Croatian `cr1`, Romanian `ro1`, Greek `gr1`/`gr2`, Hungarian `hu1`,
  Icelandic `ic1`, Swedish `sw1`/`sw2`, Estonian `ee1`,
  Lithuanian `lt1`/`lt2`, Breton `bz1`.
- **Asian:** Mandarin `cn1`, Japanese `jp1`–`jp3`, Korean `hn1`, Hindi
  `in1`/`in2`, Telugu `tl1`, Indonesian `id1`, Malay `ma1`.
- **Other:** Arabic `ar1`/`ar2`, Hebrew `hb1`/`hb2`, Persian `ir1`,
  Turkish `tr1`/`tr2`, Afrikaans `af1`, Maori `nz1`, Classical Latin
  `la1`.

Download per voice:

```
curl -L -o us1 \
  https://raw.githubusercontent.com/numediart/MBROLA-voices/master/data/us1/us1
```

Install to `/usr/local/share/mbrola/<voice>/<voice>`. The mbrola
command accepts the full binary path or (if `MBROLA_PATH` is set) a
shortened form. For Nadir_Singleton we use absolute paths — no
environment-variable plumbing.

Primary voices for us: **`us1`** (female) as the main sung vocal, `us2`
or `us3` as the male counterpart. Both 16 kHz, both sharing a phoneme
set derivable from CMUdict.

## The `.pho` file format

ASCII, line-oriented, one phoneme per line. Comments start with `;`.

Syntax of a non-comment line:

```
<phoneme> <duration_ms> [<pct1> <hz1> [<pct2> <hz2> [...]]]
```

- `phoneme`: symbol from the voice's SAMPA inventory. `_` is reserved
  for silence.
- `duration_ms`: integer ms the phoneme occupies.
- pitch points: zero or more `(position, frequency)` pairs. Position
  is 0–100 (% of phoneme duration); frequency is Hz (float allowed).

Canonical snippet from the MBROLA README:

```
_ 100
h 80 0 100 50 120 100 110
```

100 ms silence, then /h/ for 80 ms with three pitch targets — 100 Hz
at start, 120 Hz at midpoint, 110 Hz at end. MBROLA interpolates
linearly between targets and linearly across phoneme boundaries
(piecewise-linear pitch).

Rules and edge cases:

1. **Missing pitch points.** Line `phoneme duration` with no targets —
   engine holds the last specified pitch flat until the next explicit
   target.
2. **Unvoiced phonemes.** F0 is silently discarded on voiceless
   segments (`s f k t p` etc.), but pitch points on them still
   participate in the piecewise-linear interpolation for neighboring
   voiced phonemes.
3. **Multiple points inside one phoneme.** Legal and essential for
   singing. For vibrato or portamento inside a held vowel you may want
   10–20 points.
4. **Silence.** Always bracket utterances with `_ 100` at start and
   end — MBROLA needs lead-in/out for its first and last diphone.
5. **In-file directives.** `;;T=` and `;;F=` override time/frequency
   ratios mid-stream. Rarely useful; prefer CLI flags.
6. **Duration limits.** Per-phoneme cap scales inversely with pitch
   (~7.5 s at 133 Hz, ~3.75 s at 266 Hz). Split long notes into
   consecutive identical phonemes if approaching the cap.

## Invocation

```
mbrola [OPTIONS] <voicedb> <input.pho> <output.{wav|au|aiff|raw}>
```

Example:

```
mbrola /usr/local/share/mbrola/us1/us1 nadir.pho nadir.wav
```

Piping with `-` for stdin/stdout:

```
cat nadir.pho | mbrola /usr/local/share/mbrola/us1/us1 - -.raw > nadir.raw
```

`.wav` cannot be piped to stdout (requires seek to backfill the
data-chunk size). For piped use emit `.raw` (headerless s16le mono) or
`.au`. We use `.raw` with out-of-band metadata.

All output is **mono**. No stereo option. Sample rate is fixed by the
voice (16 kHz for `us1`/`us2`/`us3`/`en1` and most others; 22.05 kHz
for `de5`/`de6`). Output is always 16-bit signed linear PCM.

Flags:

- `-v VR` — volume ratio (float). Clips hard at ±32767. Use 0.8 for
  headroom.
- `-f FR` — frequency ratio; multiplies every pitch target. Useful for
  key transposition without rewriting `.pho`.
- `-t TR` — time ratio; multiplies every duration. We prefer to bake
  durations into the `.pho` and keep `-t 1.0`.
- `-l VF` — voice frequency / vocal-tract length scaling. Shifts
  formants independently of F0 — niche.
- `-e` — ignore unknown-diphone errors (synthesize silence).
- `-i` — print database info and exit.
- `-d` — dump the diphone list and exit.
- `-R "old1 new1 ..."` — phoneme rename list at load time.
- `-C "src1 dst1 ..."` — phoneme clone list for missing symbols.

## SAMPA phoneme inventory

SAMPA (Speech Assessment Methods Phonetic Alphabet) is an ASCII
transliteration of IPA — phoneme sequences typable on a US keyboard.
Each MBROLA voice defines its **own** inventory; voices are not
interchangeable at the symbol level.

`us1`/`us2`/`us3` (American English) inventory:

- Consonants: `p p_h t t_h 4 k k_h b d g f v s z S Z T D tS dZ m n N l r j w h`.
  `p_h/t_h/k_h` are aspirated allophones, `4` is the alveolar tap,
  `N` is /ŋ/, `S`/`Z` are /ʃ ʒ/, `tS`/`dZ` are /tʃ dʒ/, `T`/`D` are
  /θ ð/.
- Vowels: `i I E { A V O U u @ r=` plus diphthongs `EI AI OI @U aU`.
  `{` is /æ/ (TRAP), `@` is schwa, `r=` is the syllabic rhotic (BIRD).

`en1` (British English, MRPA-influenced):

- Monophthongs (length-marked): `i: I e { A: Q O: U u: V 3: @`
- Diphthongs: `eI aI OI @U aU I@ e@ U@`
- No glottal stop `?` or velar fricative `x`.

Crucial operational point: **`us1` and `en1` do not share the same
vowel inventory.** A `.pho` file is voice-specific. Per-voice charts
live at `https://github.com/numediart/MBROLA-voices/blob/master/data/<voice>/README.txt`.

Nadir_Singleton standardizes on `us1` / `us2` inventories for English,
with a mapping table in `python/nadir-lyric-g2p/voice_tables/us1.json`.

## G2P (grapheme-to-phoneme)

MBROLA does not do G2P. It does not know what the letter "a" sounds
like; it only renders /A/ given /A/ as input. Something upstream must
produce the phoneme sequence.

Three standard approaches:

1. **eSpeak-NG as G2P front-end.** eSpeak emits MBROLA-format `.pho`
   via `espeak-ng -v mb-us1 -q --pho --phonout=out.pho "your text"`.
   Canonical integration path; eSpeak ships phoneme-remap tables in
   `espeak-data/mbrola_ph/`.
2. **CMUdict → ARPAbet → SAMPA.** CMU Pronouncing Dictionary covers
   ~134k English words with ARPAbet; map ARPAbet→SAMPA statically.
   Fallback for OOV words via letter-to-sound rules or small LSTM G2P.
3. **Hand-authored lexicon.** Curated word→SAMPA map per project. Small
   but highest-quality.

### Policy stance for Nadir_Singleton

eSpeak is **not** an allowed runtime dependency. The audio path is
Rust + MBROLA + Praat + csdr; eSpeak is not part of that chain.

eSpeak **is permitted as an offline build-time lexicon extractor**:
during song development we may run `espeak-ng -v mb-us1 -q --pho`
on candidate lyrics to generate draft `.pho` files, then hand-edit
and commit them as static assets. The built album depends on
neither eSpeak nor CMUdict at build or run time — only the
committed `.pho` templates ship. eSpeak is a dev-time code
generator, not a pipeline component.

Nadir_Singleton owns its G2P in `python/nadir-lyric-g2p`, built on
CMUdict with an ARPAbet→SAMPA mapping table targeting the `us1`
inventory. This gives us a dependency-free runtime path and full
editorial control over pronunciation of domain-specific words (planet
names, invented words, proper nouns).

## Pitch contour design

The piecewise-linear pitch contour is the only singing lever MBROLA
exposes, and it is sufficient.

**Single-note vowel.** Two points: `a 400 0 440 100 440` holds /a/ at
440 Hz for 400 ms.

**Attack transient.** Start a few Hz below target, ramp over the first
5–10 %: `a 400 0 432 8 440 100 440`.

**Portamento between notes.** Put exit pitch at end of first vowel
and entry pitch at start of second; the boundary interpolation is the
glide:

```
a 300 0 440 100 440
e 400 0 440 20 494 100 494
```

**Stepped note changes.** Hold previous pitch flat through 100%, jump
at the start of the next phoneme:

```
a 300 0 440 100 440
e 400 0 494 100 494
```

The 1-ms discontinuity is below perceptual threshold for step changes.

**Vibrato.** Approximate with N points per cycle. At 5 Hz / ±20 cents
on a 1 s /a/ at 440 Hz: ~8 points per 200 ms cycle fools the ear.
Pre-compute in `pho-emitter` — do not type by hand.

**Boundary smoothing.** The engine interpolates across phonemes
automatically. For a sharp break into silence, put an explicit final
pitch point at 100% of the last voiced phoneme — this prevents
stray interpolation into `_`.

## Duration control for rhythm

Rhythm is achieved exclusively through phoneme and silence durations.
No "beat" primitive exists.

1. **Syllable timing.** Sung vowels 150–800 ms; consonants 50–150 ms.
   A 4/4 bar at 120 BPM = 2000 ms; a quarter-note syllable ~500 ms
   total (~100 ms consonant + ~400 ms vowel).
2. **Onset alignment.** The perceived beat lands on the vowel, not
   the consonant. Push consonants **before** the target time: for a
   downbeat at t=0, start /n/ at t=-100 ms so /a/ in "nadir" hits
   the beat.
3. **Silence lengths.** Short `_` 20–60 ms for phoneme-internal
   micro-pauses (word breaks with no rhythmic gap); 100–400 ms
   between phrases; ≥500 ms between sections; ≥50 ms leading silence
   mandatory at file start.
4. **Rests.** One `_` with the correct duration. Do not continue a
   pitch contour across it.

## Integration patterns

From Rust we drive `mbrola` as a subprocess: write `.pho` to stdin,
read raw PCM from stdout.

```rust
use std::process::{Command, Stdio};
use std::io::Write;

let mut child = Command::new("/usr/local/bin/mbrola")
    .arg("-e").arg("-v").arg("0.8")
    .arg("/usr/local/share/mbrola/us1/us1")
    .arg("-").arg("-.raw")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

child.stdin.as_mut().unwrap().write_all(pho_bytes)?;
drop(child.stdin.take()); // EOF so mbrola exits
let out = child.wait_with_output()?;
let pcm_s16le_16k_mono: Vec<i16> = out.stdout
    .chunks_exact(2)
    .map(|c| i16::from_le_bytes([c[0], c[1]]))
    .collect();
```

Downstream: 16 kHz → 48 kHz via csdr. Because 16 → 48 is exactly ×3
use integer `fir_interpolate` with a tight anti-imaging low-pass:

```
mbrola ... -.raw \
  | csdr convert_s16_f \
  | csdr fir_interpolate_cc 3 \
  | csdr convert_f_s16 \
  > vocal_48k.raw
```

For non-integer ratios (22.05 kHz voices) use
`fractional_decimator_ff` or route through SoX. Prefer keeping the
chain in csdr to avoid extra disk round-trips.

## Known pitfalls

1. **Voice licensing.** Engine is AGPL; voice databases each ship their
   own license. Most voices are "free for non-commercial use only,"
   inherited from Babel Technology SA and similar donors. Before any
   commercial release, audit `license.txt` per voice. Options for
   commercial use: (a) keep releases non-commercial, (b) license
   commercially from rights holder, (c) swap to a commercial-friendly
   voice. Release-gating concern, not build-time.
2. **Sample-rate mismatch.** Voices are 16 kHz (most) or 22.05 kHz (some
   German). Nadir_Singleton masters at 48 kHz. Always resample before
   summing into the bus — misreading 16 kHz raw as 48 kHz produces a
   chipmunked vocal. Bake the sample rate into metadata in Rust.
3. **Per-voice phoneme differences.** `I` in `us1` vs `I` in `en1` sound
   different; some symbols exist only in one inventory. `.pho` is
   voice-specific. `pho-emitter` reads a voice table at generation time.
4. **Required leading silence.** MBROLA needs `_` at the start (≥50 ms)
   or the first diphone is truncated with an audible click. Same at
   tail — always end with `_`.
5. **No stereo.** Mono only. Build stereo downstream (double-tracking,
   wide-mono reverb, Haas delay).
6. **arm64 / macOS build quirks.** Patch the `swab` collision in
   `Misc/common.h`; enable `CFLAGS += -DTARGET_OS_MAC`. No Homebrew
   formula as of April 2026; we build from source and check the binary
   into `tools/mbrola-bin/`.
7. **Duration cap tied to F0.** Long notes at low pitch can exceed the
   internal buffer (~7.5 s at 133 Hz). Split into multiple consecutive
   identical phonemes for notes >5 s.
8. **No text normalization.** Numbers, abbreviations, and foreign words
   must be normalized before G2P ("1984" → "nineteen eighty four").

## How we use it in Nadir_Singleton

MBROLA is the **primary vocal source**. Pipeline per sung line:

1. **Lyric** — plain text, one syllable per token, annotated with bar,
   beat, duration, target pitch.
2. **G2P** — `python/nadir-lyric-g2p` → SAMPA (us1) via CMUdict +
   ARPAbet→SAMPA + project lexicon overrides.
3. **pho-emitter** — Rust crate; consumes `(phonemes, timing,
   pitch_spec)`, emits `.pho`. Applies vibrato, attack ramps,
   portamento/stepped transitions, phrase silences. F0 locked to the
   song's scale; microtonal targets preserved.
4. **MBROLA** — subprocess; stdin `.pho`, stdout raw s16le 16 kHz mono.
5. **Praat PSOLA clean-up** — optional; smooths pitch, fixes residual
   diphone-boundary artifacts.
6. **csdr upsample** — 16 → 48 kHz via ×3 interpolation + anti-imaging
   FIR.
7. **Bus** — summed into vocal bus, optionally with
   doubled/harmonized MBROLA layers.

Everything except MBROLA is code we own. MBROLA is the single
external binary per voice line.

## Canonical example — singing "nadir"

Target: sing "nadir" in A minor across two notes — /eI/ on A4
(440 Hz), /I/ rising to C5 (523.25 Hz), then `r=` falling back to A4
through Bb4 (466.16 Hz).

SAMPA (us1): `n eI d I r=`. `us1` has no centering diphthong `I@`;
we use `I` + syllabic rhotic `r=`, the native American realization
of "dir" in "nadir."

Timing at 100 BPM: /n/ 100 ms onset (pre-beat), /eI/ 400 ms (beat 1),
/d/ 80 ms bridge, /I/ 200 ms rising, `r=` 520 ms held+falling.

`nadir.pho`:

```
; Nadir_Singleton — canonical sung "nadir"
; voice: us1, key: A minor, tempo: 100 BPM
; A4 = 440, Bb4 = 466.16, C5 = 523.25
_ 100
n 100 0 440 100 440
eI 400 0 440 8 440 85 440 100 440
d 80 0 440 100 523.25
I 200 0 523.25 100 523.25
r= 520 0 523.25 40 523.25 70 466.16 100 440
_ 200
```

Contour notes:

- Leading `_ 100` is mandatory lead-in silence.
- `n` holds A4 flat (voiced — the nasal hum carries F0).
- `eI` is dead flat on A4; a scooped take would insert `50 435` for
  a human feel.
- `d` is the pitch bridge: linear interpolation 440 → 523.25 Hz
  across its 80 ms.
- `I` holds C5 flat for 200 ms.
- `r=` holds C5 for 40%, descends through Bb4 at 70%, lands on A4
  at 100% — a ~300 ms blues/jazz fall.
- Trailing `_ 200` for safe tail.

Render:

```
mbrola -e -v 0.8 \
  /usr/local/share/mbrola/us1/us1 \
  nadir.pho \
  nadir_16k.wav
```

Upsample to 48 kHz for the project bus:

```
mbrola -e -v 0.8 /usr/local/share/mbrola/us1/us1 nadir.pho -.raw \
  | csdr convert_s16_f \
  | csdr fir_interpolate_cc 3 \
  | csdr convert_f_s16 \
  > nadir_48k.raw
```

Expect a recognizable, synthetic-but-listenable female rendition of
"nadir" with a tasteful C5→A4 fall on the final rhotic.

## References

- Engine: https://github.com/numediart/MBROLA
- Voices: https://github.com/numediart/MBROLA-voices
- Database builder: https://github.com/numediart/MBROLATOR
- Dutoit, T., Pagel, V., Pierret, N., Bataille, F., Van der Vrecken, O.
  "The MBROLA project: towards a set of high quality speech
  synthesizers free of use for non commercial purposes." ICSLP 1996,
  pp. 1393–1396. DOI 10.21437/ICSLP.1996-356.
  https://www.isca-archive.org/icslp_1996/dutoit96_icslp.html
- Bozkurt, Dutoit, Prudon, d'Alessandro, "From MBROLA to NU-MBROLA"
  (non-uniform units extension).
- numediart 2018 AGPL release (24 Oct 2018):
  https://github.com/numediart/MBROLA
- eSpeak-NG MBROLA docs:
  https://github.com/espeak-ng/espeak-ng/blob/master/docs/mbrola.md
- UCL SPSCI Lab 8 (concatenative synthesis):
  https://www.phon.ucl.ac.uk/courses/spsci/spc/lab8.html
- macOS `swab` build issue: https://github.com/numediart/MBROLA/issues/9
- Wikipedia: https://en.wikipedia.org/wiki/MBROLA
