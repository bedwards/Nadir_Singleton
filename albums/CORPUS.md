# Nadir_Singleton Corpus

Twenty albums, 14–24 tracks each. Narrative arc within and across.

| # | Slug | Title | Motif palette | Range |
|---|------|-------|---------------|-------|
| 01 | `01_horizon_salts` | Horizon Salts | first utterance, dawn-voicing | 14 |
| 02 | `02_tin_pan_fathom` | Tin Pan Fathom | TPA harmonic turns on diphones | 18 |
| 03 | `03_the_spectral_fair` | The Spectral Fair | carnival eGeMAPS vectors | 16 |
| 04 | `04_vad_epistles` | VAD Epistles | letter-form tracks | 14 |
| 05 | `05_phoneme_monastery` | Phoneme Monastery | vowel drones, chant | 15 |
| 06 | `06_csdr_weather` | csdr Weather | DSP-only beds | 14 |
| 07 | `07_praat_lieder` | Praat Lieder | minimal art-song | 16 |
| 08 | `08_mbrola_cabaret` | MBROLA Cabaret | ragtime/cabaret forms | 18 |
| 09 | `09_formant_gardens` | Formant Gardens | botanical ambient | 14 |
| 10 | `10_the_cent_maze` | The Cent Maze | microtonal | 16 |
| 11 | `11_voicing_fraction` | Voicing Fraction | pop hooks | 18 |
| 12 | `12_diphone_drift` | Diphone Drift | tape-like slow change | 15 |
| 13 | `13_egemaps_tarot` | eGeMAPS Tarot | 22 feature-family archetypes | 22 |
| 14 | `14_silero_rooms` | Silero Rooms | onset-driven percussive | 16 |
| 15 | `15_fir_psalms` | FIR Psalms | csdr-shaped hymns | 14 |
| 16 | `16_pitch_pilgrimage` | Pitch Pilgrimage | modal per track | 16 |
| 17 | `17_plosive_letters` | Plosive Letters | consonant-forward | 14 |
| 18 | `18_dorian_weather` | Dorian Weather | modal pop | 16 |
| 19 | `19_singleton_suite` | Singleton Suite | long-form, 24 connected | 24 |
| 20 | `20_nadir` | Nadir | coda, de-resolution | 14 |

## Cross-album motif catalogue

Motifs are named pairs of `(F0 contour class, DSP signature)` used across albums.
Each song manifest records which motifs it carries. Seed motifs:

- `m.dawn_utterance` — rising 3rd, then plateau; csdr band-limit 80–3.2 kHz; first used in 01.
- `m.tin_pan_turnaround` — ii-V-I on scale degrees; ring-mod ghost carrier at 40 Hz; 02.
- `m.feature_vector_waltz` — swung 3/4, each beat one eGeMAPS family; 03.
- `m.letter_cadence` — silence-padded final syllable; 04.
- `m.vowel_drone` — held `a` or `o` 8 s, micro-vibrato; 05, returns in 19.
- `m.room_impulse` — csdr Dirac-click bed, vary IR length; 06, 14.
- `m.whisper_psola` — Praat PSOLA de-voicing of MBROLA; 07.
- `m.cabaret_jump` — ragtime left-hand stride transposed to scale grid; 08.
- `m.pollen_formant` — slow formant-shift sweep; 09.
- `m.quarter_tone_approach` — 50¢ neighbour-tone; 10.
- `m.hook` — 4-bar refrain, lyric repetition; 11, 18.
- `m.drift_delta` — 0.2¢ per bar pitch crawl; 12.
- `m.tarot_archetype` — one eGeMAPS family per track; 13.
- `m.onset_volley` — 16th-note VAD onsets at threshold 0.2; 14.
- `m.fir_blessing` — cascaded bandpass chain blessing the vocal; 15.
- `m.modal_shift` — scale reselection mid-song; 16.
- `m.plosive_rhythm` — `p`/`t`/`k` as percussion; 17.
- `m.dorian_mirror` — melody mirrored around the 5th; 18.
- `m.singleton_spine` — recurring 12-note row across 24 tracks of album 19.
- `m.nadir_silence` — calibrated silences encoding Morse of "Nadir"; 20.

Motif updates are PRs against this file with a `motif/<id>` branch label.
