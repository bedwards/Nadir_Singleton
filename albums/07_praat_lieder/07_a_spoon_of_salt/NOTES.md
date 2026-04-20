# 07 A Spoon of Salt — notes

## Why this slot
The deliberate reprise inside the album 07 cycle. Album 01 Horizon Salts
opened the corpus with a salt-on-the-tongue utterance; here we hold the
salt in a spoon instead of on the tongue, and we no longer say the
original phrase out loud (it appears in the lyric as memory: "the first
salt / the small salt / on the tongue of the morning"). The melodic
contour at that remembered line carries `m.dawn_utterance`.

## Form and motif deployment
Through-composed, 18 bars in A minor at 56 bpm, 4/4. A minor is the same
key as album 01 track 01 First Salt, deliberately. Two motifs:
`m.whisper_psola` across the whole track, and `m.dawn_utterance` on the
three remembered lines (bars 9–12) — a rising minor third from A3 to C4,
then plateau on C4. The plateau is compressed to two bars rather than the
four bars of the original utterance, because this is a reference, not a
statement.

## Praat treatment
Leans on **`psola_retarget.praat`** from `nadir-praat-scripts`. Pitch
extraction autocorrelation (`ac`), floor 65 Hz, ceiling 1000 Hz,
very-accurate on. Target median 215 Hz. Range factor 0.70 — tight but
not the tightest. Formant shift ratio 1.00. The reprise lines use a
second PSOLA pass to re-pitch those bars specifically onto the
A3 → C4 contour, leaving the surrounding verse at the flattened whisper.

## csdr graph shape
`band_limit` preset at -24 dB, 200–2400 Hz. The reprise bars (9–12) fade
the bed 3 dB quieter to let the melodic gesture land clean. No reverb.

## G2P / pronunciation hints
- "salt" as /s O l t/ — match the /O/ length of album 01 track 01.
- "spoon" as /s p u n/ — clean, long /u/.
- "tongue" as /t V N/ — no /g/ at the end.
- "morning" as /'m O r n I N/ — match the album 01 pronunciation
  exactly; this is a quoted line.

## openSMILE gate
Voicing fraction in [0.32, 0.55]. Pitch-error RMS 2 cents ceiling, with
an additional gate on bars 9–12: the reprise contour must land within
1.5 cents of the A3 → C4 template measured from album 01 track 01.
Loudness -18 LUFS integrated.
