# 05 The Middle Register Vacates — notes

## Descent step
A spectral hollowing. The csdr bed uses a `fir_inverse_bandstop` centred on 400–1600 Hz — the melodic mid-band is suppressed by 18 dB. What remains: low rumble (80–180 Hz) and thin top (3–8 kHz). The vocal, too, is pulled to its extremes; the `custom` scale for this track is `[0, 1, 11]` — tonic, flat-2, flat-7 — no mid-scale tones.

## Embedding of `m.nadir_silence`
The missing middle IS the silence here. We still insert 4 × 1400 ms calibrated silences at the bar boundaries 3, 6, 9, 12. The room-with-no-furniture metaphor maps directly to this.

## Reprises
- `m.fir_blessing` (album 15): the hollowing bandstop is itself a cascaded FIR — we bless the vocal by taking from it, inverse of the hymn.
- `m.pollen_formant` (album 09): formant-sweep drags F2 from its natural 1500 Hz down below 300 Hz across the song — the vowel colours become low and closed.

## Pronunciation hints
- "middle" → `m I d l=`; syllabic `l=` is elongated 700 ms on a single low pitch (≈ 110 Hz).
- "ceiling" → `s i: l I N`; voiced `N` held 900 ms, high formants only.
- "furniture" → `f 3:` ... full SAMPA `f 3: n I tS @`. Bars 11–12 we truncate after `f 3:` — the word is unfinished as the track dies.

## openSMILE gates
- voicing fraction: target 0.40, floor 0.30.
- pitch error RMS ceiling: 18.0 cents.
- loudness integrated: -20.0 LUFS.
- spectral centroid (eGeMAPSv02 `spectralCentroid_sma3nz_amean`): expected bimodal; we log but do not gate.
