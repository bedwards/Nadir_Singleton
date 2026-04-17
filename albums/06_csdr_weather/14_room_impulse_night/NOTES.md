# 14 Room Impulse Night — notes

## Why this slot
Closer. We take the album's motif `m.room_impulse` at face value and build the track around it exclusively: dirac clicks with long IR tails, one per eight bars, decaying into near-silence. The room itself is now the weather. Callback to 01 Morning FIR Calm (opener) — the album bookends with deployed-but-sparse impulses.

## No-vocal contract (album exception)
Album 06 is the DSP-only exception. `mbrola_voice = ""`. Five-tool ledger:
- MBROLA: zero-amplitude silence stem. Important here because the track IS mostly silence — the silence-anchor role is nearly coincident with the artistic content.
- Praat: KlattGrid texture stem; single held `i:` vowel formants at -42 dB, barely on the noise floor, a ghost of a voice.
- Silero-VAD: audit voicing fraction ~0; this is our sharpest gate — if the Praat `i:` leaks we'd see voicing rise, and we regen Praat with further bandwidth narrowing.
- openSMILE: spectral_slope, loudness, band_limited_rms.
- csdr: the primary output.

## csdr graph shape
Primary = `presets/dirac_impulse_bed.toml`. Variation `dirac_impulse_bed_coda_vlong`: `peaks_fir_cc 16384` (very long IR, ~340 ms tail at 48 kHz; longest on album), driven by one click per eight bars from the `yes_f 1.0 1` generator gated by external scheduler, followed by `gain_ff 0.3`. Texture: `shaped_noise_bed` narrowed to `-0.05 0.05` at -30 dB — a bare room hiss. Sample source: `fir_cascade` to shape the IR tail's timbre.

## Motif deployment
`m.room_impulse` is the entire piece. Four clicks total across 32 bars (bars 1, 9, 17, 25). IR length per click is approximately 2 s (engineered via extra-long `peaks_fir_cc` plus an overlapping deemphasis pole). This is the longest IR time the album carries.

## Reprise
Reprises the opener 01 Morning FIR Calm but inverts the deployment: 01 had IR inside an FIR-dominated bed; 14 has the bed reduced almost to silence so the IR stands alone.

## openSMILE audit gates
- spectral_slope: -9 dB/oct +/- 2 dB (matches opener).
- loudness integrated: -25 LUFS +/- 1 dB (quietest track on album; softest in entire sequence).
- band_limited_rms (broadband): below -28 dBFS.
- voicing_fraction: 0.02 ceiling — sharpest gate on album.
