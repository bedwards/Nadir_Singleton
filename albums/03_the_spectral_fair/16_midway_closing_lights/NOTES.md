# Midway Closing Lights — notes

The album's coda. Every family the fair visited gets one last lantern — a
single beat each — before the waltz is tucked away. We name loudness, jitter,
shimmer (as "shine"), harmony (HNR), tone, the hawkers, the horses (carousel),
and the bell (voicing probability ring).

## Slot rationale
A closer should echo the opener. We return to G major (track 01's key),
reinstate `m.dawn_utterance` briefly for one bar, then settle into a slower
120 bpm so the waltz slows as the fair empties. Every earlier motif gets a
one-beat nod.

## Motif deployment
- 3/4 at 120 bpm, swing 60 %.
- A sections: each bar lights one family's lantern in order of appearance
  (F0, loudness, slope, HNR, jitter, shimmer, MFCC, flux).
- A' sections: remaining families (formant, voicing, alpha, Hammarberg, H1-H2,
  bandwidth, pitch percentile, and the closing bell).
- B: all lanterns held briefly for 4 bars; the granular bed decays to 0.
- Final 4 bars of A: unaccompanied vox on G, the fair is closed.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> ring_mod_multi (all album carriers cascaded; each decays in turn)
  -> granular_texture (grain density decays bar by bar to zero)
  -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "lantern" as `'l { n t @ n` — soft final n.
- "midway" as `'m I d w eI`, "mellow" as `'m E l oU`.
- Avoid plosive pile-up on "tucked"; sing as `'t V k t` with light final t.

## openSMILE gates
- F0 RMS error < 2 cents.
- Loudness mean decreases monotonically by >= 0.4 sone over the track.
- Voicing fraction > 0.65 overall.
- Spectral flux mean at final bar < 30 % of opening bar (the lanterns dim).
