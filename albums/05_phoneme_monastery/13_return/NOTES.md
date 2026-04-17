# 13 Return — notes

## Why this slot
After `12_silence_rule_broken`, the rule is restored. Not punitively —
monastic discipline resolves transgression through re-entry, not
exclusion. The brother who spoke rejoins the brotherhood of /a/, /o/,
/u/. The tempo recovers to a meditative 50 bpm; aeolian is home. The
track is longer than usual (20 bars) because re-entry takes time.

## Form and motif deployment
AB form, 20 bars at 50 bpm. Section A (bars 1–10): /a/ alone, in
progressively longer and denser held repetitions — the single vowel
re-builds its ground. "we return / we return to the vowel" is sung at
the centre of A; the rising figure on "return" is a reprise of
`m.dawn_utterance` (C3 → E3, a minor third scaled low). Section B
(bars 11–20): the full brotherhood /a/ /o/ /u/ returns, and the phrase
"we return to the drone" closes with `m.letter_cadence` on the final
"drone" syllable — a silence-padded cadence that rhymes with compline
before the bells. `m.vowel_drone` is carried by every held vowel.

## csdr graph shape
`fir_cascade` per vowel. `dirac_impulse_bed` at 0.5 Hz — the bell is
back to its matins cadence, as if dawn is being re-enacted at a
human-made hour. This is deliberate: return is dawn out of schedule.

## G2P / pronunciation hints
- `a` → SAMPA `a`; durations progress 3 s, 4 s, 5 s, 6 s across the
  four openings.
- `o`, `u` as established.
- "we" → `w i:`; opens each return-phrase softly.
- "return" → `r I t @: n`; the `@:` is lengthened to 1.2 s, and rises
  from C3 to E3 — the `m.dawn_utterance` fragment.
- "vowel" → `v aU @ l`; the `aU` deliberately grazes the /a/ drone.
- "drone" → `d r @U n`; terminal `n` sustained 2 s and silence-padded
  1.5 s — `m.letter_cadence`.

## openSMILE gates
- voicing fraction: target 0.70, floor 0.60.
- pitch error RMS ceiling: 3.0 cents.
- loudness integrated: -20.0 LUFS.
- F0 stddev semitones across held vowels: maximum 0.4 ST.
- re-entry continuity: subjective gate — we regen if Silero-VAD
  reports any abrupt onset after a drone (discontinuity between
  return and drone means the brother hasn't actually returned).
