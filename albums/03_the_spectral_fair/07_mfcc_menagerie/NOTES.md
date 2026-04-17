# MFCC Menagerie — notes

eGeMAPSv02 publishes MFCC 1-4 as LLDs. We give each of the four coefficients
its own creature in the menagerie, so the song has four melodic motifs rotating
through the A sections, one per MFCC.

## Slot rationale
After two tracks on micro-instability, the album pivots to timbre. MFCC is the
most compact timbral descriptor we have; placing it at 07 marks the middle of
the record and the transition from monostate families (pitch, loudness, slope)
to multidimensional ones (MFCC, formants, harmonics).

## Motif deployment
- 3/4 waltz, 132 bpm, swing 58 %.
- Beat 1 = MFCC1 mean (timbre tilt). Beat 2 = MFCC2 mean. Beat 3 = MFCC3 mean.
- A' and A'' cycle through the four coefficients by shifting the beat roles.
- B section (bars 17-20) is MFCC4 only: a soloed coefficient against bed.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> splitter (4 paths)
    path1: FIR shaped to MFCC1 target -> creature_lion
    path2: FIR shaped to MFCC2 target -> creature_linnet
    path3: FIR shaped to MFCC3 target -> creature_mouse
    path4: FIR shaped to MFCC4 target -> creature_tiger
  mixer(4 paths, bar-assigned gains) -> ring_mod_multi -> bed
  granular_texture -> texture
  mixer -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "coefficient" as `k oU @ 'f I S @ n t` — keep the cluster light.
- "cepstral" as `'s E p s t r @ l` — allow a schwa between p and s.
- "menagerie" as `m @ 'n { dZ @ r i` — open final i.

## openSMILE gates
- MFCC1 RMS delta < 0.3 vs reference (timbre preserved).
- MFCC2-4 RMS delta < 0.4.
- F0 RMS error < 2 cents.
- Voicing fraction > 0.7.
