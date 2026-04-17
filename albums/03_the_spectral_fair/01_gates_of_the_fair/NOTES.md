# Gates of the Fair — notes

We open album 03 on the turnstile. The first family we meet is F0 stats —
mean, median, percentile, range — so the song counts arrivals and names the
shape of the day's voice before any other attraction is entered.

## Slot rationale
The opener has to hand the listener the album's signature. `m.feature_vector_waltz`
lands immediately in 3/4 at 132 bpm; every downbeat sits on a new F0-stats
functional (mean on 1, median on 2, range on 3). `m.dawn_utterance` returns for
one bar as a handshake with album 01.

## Motif deployment
- Bars 1-8 (A): vocal traces a low-to-high arc per bar, each beat one pitch
  statistic. Downbeat = mean F0. Beat 2 = median. Beat 3 = 80th percentile.
- Bars 9-16 (A'): same contour, transposed up a fourth so the percentile climbs.
- Bars 17-20 (B): inverted contour — percentiles descend while the mean holds.
- Bars 21-24 (A): return, resolved on the tonic G.

## csdr graph shape
```
MBROLA vox -> csdr convert_s16_f -> fir_interpolate_48k -> shift_addition_cc
  |-> ring_mod (carrier = mean F0 of bar) -> bed
  |-> granular_texture (grain len 40 ms, pitch pattern = bar's percentile)
  '-> mixer (vox 0.7, bed 0.2, texture 0.1) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "three" as `T r i:` — lingual fricative kept short, avoid smearing.
- "median" as `'m i d i @ n` — open the schwa, MBROLA us1 handles it cleanly.
- "colours" as `'k V l @ z` — soft r; no British rhotic.

## openSMILE gates
- F0 RMS error < 2 cents (eGeMAPSv02 primary).
- F0 mean within 8 % of planned arc per bar (custom functional).
- Voicing fraction > 0.7 on A sections, > 0.55 on B.
- Loudness mean 0.9-1.1 sone (opener headroom).
