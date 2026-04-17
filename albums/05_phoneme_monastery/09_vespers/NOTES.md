# 09 Vespers — notes

## Why this slot
Vespers is the evening office; light is candle-thin, the community
gathers after work. D dorian softens the darkness of the afternoon
phrygian (`08_vigil`) and prepares compline's deeper minor. 3/4 meter
is the album's first triple — it suggests the gentle rocking of a
candle flame. `us1` returns for brightness at the long day's end.

## Form and motif deployment
Verse/refrain over 18 bars at 54 bpm. Verses alternate /o/ drone and
/a/ drone (8 s each). Refrain: "the candle holds …" recited on D4 with
the final syllable drifting down to A3 — this is the `m.tin_pan_turnaround`
reprise, compressed to a minor ii→V→i scale-degree figure on the phrase
"holds the hour/room". The turnaround is so quiet under the drone that
only a careful listener identifies it as TPA vocabulary smuggled into
the cloister.

## csdr graph shape
`fir_cascade` switched per vowel. `dirac_impulse_bed` at 0.5 Hz —
vesper bells ringing on their own unhurried clock. A very faint
`ring_mod` ghost carrier at 40 Hz on the "turnaround" refrain is the
signature of `m.tin_pan_turnaround` (CORPUS.md: "ring-mod ghost
carrier at 40 Hz") — but run at -30 dB so it colors rather than sings.

## G2P / pronunciation hints
- `o` → SAMPA `O`; 8 s hold on first occurrence, 5 s on `o o o`
  repetitions.
- `a` → SAMPA `a`; same durations as the paired /o/ structure.
- "candle" → `k a n d @ l`; first-syllable `a` rhymes with the drone.
- "holds" → `h @U l d z`; terminal unvoiced.
- "hour" → `aU @`.
- "room" → `r u: m`; the `u:` briefly visits the third vowel of the
  brotherhood before being occluded by `m`.

## openSMILE gates
- voicing fraction: target 0.70, floor 0.60.
- pitch error RMS ceiling: 3.0 cents.
- loudness integrated: -20.0 LUFS.
- spectral flux: maximum 0.025 per frame.
