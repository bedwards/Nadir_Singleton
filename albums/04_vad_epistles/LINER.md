# VAD Epistles

_Album 4 of 20 in the Nadir_Singleton corpus._

fourteen letters; each ends in calibrated silence announcing the next.

## Motif palette
letter-form tracks. The central motif is `m.letter_cadence`: every final syllable
is padded with silence whose duration we calibrate per-track from the Silero-VAD
probability track. The silence is not empty — it is the envelope of the letter,
sealed.

## Tracks (planned)

1. To the Lighthouse — a thank-you card sent back to album 02.
2. From the Broken Metronome — the metronome writes us.
3. To Our Younger Self — addressed to the singleton at first utterance.
4. To a Stranger on a Train — a letter we will never deliver.
5. To the Engineer of Diphones — to the builder of the first us1 database.
6. To the Morning We First Woke — back to album 01 dawn.
7. To the Listener — second-person, intimate.
8. To the Room Without Us — addressed to an empty room.
9. To the Unreturned Letter — a letter about a letter we never got back.
10. To the One We Could Not Reach — distance, graceful.
11. To the Postmaster — meta, a complaint both earnest and small.
12. To the Night Watchman — a goodnight to a character from album 02.
13. To the Silence Between — to the calibrated gap itself.
14. To Whom It May Concern — the last letter, to everyone and no one.

## Production notes

- Intimate loudness: -16 LUFS integrated (quieter than the -14 LUFS of 01-03).
- Vocal tessitura default 150-420 Hz. Letter-voice sits lower than arias.
- Bed is never a full texture; `shaped_noise_bed` or `deemphasis_chain` only.
- Silero-VAD onset threshold 0.35 per track, tuned for breath points between
  clauses. Final-syllable silence is measured off the VAD fall-edge, not by
  lyric scansion.
- Keys and bpms vary so each letter carries its own weather.
- `m.letter_cadence` is the only required motif; reprises from albums 01-02
  are cited per-track in NOTES.md.

