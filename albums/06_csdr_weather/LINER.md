# csdr Weather

_Album 6 of 20 in the Nadir_Singleton corpus._

weather made entirely of csdr pipes; no voice, only shaped air.

## Motif palette
DSP-only beds

## Tracks (planned)

1. Morning FIR Calm — dawn air; FIR cascade warming up.
2. Patterned Drizzle — first drops, 16th-note dirac impulses.
3. Low-Shelf Rain — steady wash under a deep low-shelf.
4. Pink Snow — broadband pink-spectrum snowfall.
5. Band-Limit Thunder — sub-200 Hz distant front.
6. Dorian Squall — 6/8 wind on four dorian-tuned ring-mod carriers.
7. Granular Snow — 20 parallel bandpass branches, grain-windowed.
8. Phase Hail — 32nd-note metallic impulse curtain on side-B opener.
9. Hemlock Fog — long near-static drone; album's longest track.
10. Ring-Mod Gale — AABA, carriers sweep across dorian range.
11. FFT Mist — explicit FFT comb brushing, slow mist breaths.
12. Deemphasis Dusk — deep deemphasis, spectrum darkens at nightfall.
13. Dirac Sleet — mixed short/long IR impulses, icy patter.
14. Room Impulse Night — coda; `m.room_impulse` bare, long IR into silence.

## Production notes

Album 06 is the DSP-only exception across the corpus: no MBROLA vocal part on
any track. `mbrola_voice = ""` in every manifest. The five-tool render-lock
constraint (`RenderLock::verify_all_five_tools`) is nonetheless honoured:

- **MBROLA** still runs, emitting a zero-amplitude silence stem that acts as the
  48 kHz sample-rate sync anchor (via csdr `rational_resampler_ff 3 1`).
- **Praat** generates a `KlattGrid` texture stem per track, used as one of the
  quieter bed colours (typically -30 to -42 dB).
- **Silero-VAD** audits the silence anchor: expected voicing fraction ~0 is a
  sanity check that no voice leaked into any stem.
- **openSMILE** audits the mix with `eGeMAPSv02` — features of interest are
  `spectral_slope`, `loudness`, and `band_limited_rms` on chosen sub-bands.
  Voicing-fraction and pitch-error gates are not meaningful here.
- **csdr** is the primary synthesis path for every track.

Motif: every track carries `m.room_impulse` (csdr Dirac-click bed, varying IR
length from ~30 ms on `08_phase_hail` to ~2 s on `14_room_impulse_night`). IR
length is the primary narrative parameter across the album.

All applicable csdr presets from `presets/` are used somewhere on the album:
`shaped_noise_bed`, `dirac_impulse_bed`, `granular_texture`, `fir_cascade`,
`deemphasis_chain`, `ring_mod_multi`.
