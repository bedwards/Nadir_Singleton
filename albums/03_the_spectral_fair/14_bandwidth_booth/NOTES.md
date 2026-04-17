# Bandwidth Booth — notes

Formant bandwidth (`F1bandwidth_sma3nz` and siblings) is the second formant
family we pair with track 09's formant-frequency track. Narrow bandwidth =
tuned voice; wide bandwidth = breathy / damped. The booth is a tiny space
where the curtain width is the measurement.

## Slot rationale
Bandwidth wants a quiet stall, not a stage, so we slot it at 14 (between two
more theatrical siblings: 13 H1-H2 and 15 pitch percentile). The G minor key
here ties back to Track 01's G major opening, softening the album toward its
closer.

## Motif deployment
- 3/4, 130 bpm, swing 55 %.
- Beat 1 = F1 bandwidth, beat 2 = F2 bandwidth, beat 3 = F3 bandwidth.
- A sections: bandwidths widen; A' narrow; B holds midpoint.
- The closing rhyme lands on "bell" to keep us clear of heavy consonant
  clusters — MBROLA us1 handles final `l` cleanly.

## csdr graph shape
```
MBROLA vox -> convert_s16_f -> fir_interpolate
  -> Praat bandwidth script (widen / narrow F1-F3 formants per bar)
  -> ring_mod_multi (carriers = F1, F2, F3 centres)
  -> granular_texture (grain len = bandwidth / 10)
  mixer(vox, bed, texture) -> fastagc_ff -> mix
```

## G2P pronunciation hints
- "bandwidth" as `'b { n d w I T` — soft final th.
- "formant" as `'f O r m @ n t`, "ribbon" as `'r I b @ n`.
- "velvet" as `'v E l v @ t` — short schwa.

## openSMILE gates
- F1 bandwidth mean within +/- 30 Hz of plan.
- F2 bandwidth mean within +/- 50 Hz of plan.
- F3 bandwidth mean within +/- 80 Hz of plan.
- F0 RMS error < 2 cents.
