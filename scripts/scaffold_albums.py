#!/usr/bin/env python3
"""Generate 20 album directories with MANIFEST.toml and LINER.md.

Idempotent: skips existing files.
"""
from __future__ import annotations

import pathlib
import textwrap

ROOT = pathlib.Path(__file__).resolve().parents[1]
ALBUMS_DIR = ROOT / "albums"

ALBUMS = [
    ("01", "horizon_salts",      "Horizon Salts",      14, "first utterance, dawn-voicing",
     "awakening: the singleton speaks for the first time, salted with spectral dawn."),
    ("02", "tin_pan_fathom",     "Tin Pan Fathom",     18, "TPA harmonic turns on diphones",
     "vaudeville ghosts of Tin Pan Alley push diphone voices through old forms."),
    ("03", "the_spectral_fair",  "The Spectral Fair",  16, "carnival eGeMAPS vectors",
     "a fairground of feature vectors: each attraction a functional family."),
    ("04", "vad_epistles",       "VAD Epistles",       14, "letter-form tracks",
     "fourteen letters; each ends in calibrated silence announcing the next."),
    ("05", "phoneme_monastery",  "Phoneme Monastery",  15, "vowel drones, chant",
     "ritual vowel drones; a brotherhood of /a/, /o/, /u/."),
    ("06", "csdr_weather",       "csdr Weather",       14, "DSP-only beds",
     "weather made entirely of csdr pipes; no voice, only shaped air."),
    ("07", "praat_lieder",       "Praat Lieder",       16, "minimal art-song",
     "sixteen art songs, one voice, minimal bed; PSOLA as the composer."),
    ("08", "mbrola_cabaret",     "MBROLA Cabaret",     18, "ragtime/cabaret forms",
     "the singleton in a cabaret: ragtime turnarounds, knowing winks."),
    ("09", "formant_gardens",    "Formant Gardens",    14, "botanical ambient",
     "gardens tended by slow formant sweeps; vowels as flora."),
    ("10", "the_cent_maze",      "The Cent Maze",      16, "microtonal",
     "quarter-tones and smaller; a labyrinth drawn in cents."),
    ("11", "voicing_fraction",   "Voicing Fraction",   18, "pop hooks",
     "pop shapes: hooks, refrains, choruses you can hum back to us."),
    ("12", "diphone_drift",      "Diphone Drift",      15, "tape-like slow change",
     "long slow drifts, 0.2¢ per bar; the sound of time leaning."),
    ("13", "egemaps_tarot",      "eGeMAPS Tarot",      22, "22 feature-family archetypes",
     "twenty-two tracks, one per eGeMAPS family archetype; the corpus tarot."),
    ("14", "silero_rooms",       "Silero Rooms",       16, "onset-driven percussive",
     "rooms whose rhythm is decided by voice-activity onsets."),
    ("15", "fir_psalms",         "FIR Psalms",         14, "csdr-shaped hymns",
     "hymns blessed by cascaded FIR filters; liturgy of the passband."),
    ("16", "pitch_pilgrimage",   "Pitch Pilgrimage",   16, "modal per track",
     "one mode per track, a walking tour of the diatonic and beyond."),
    ("17", "plosive_letters",    "Plosive Letters",    14, "consonant-forward",
     "/p/, /t/, /k/ are percussion; the lyrics are the drum kit."),
    ("18", "dorian_weather",     "Dorian Weather",     16, "modal pop",
     "pop structures dressed in Dorian, a little rain in every chorus."),
    ("19", "singleton_suite",    "Singleton Suite",    24, "long-form, 24 connected tracks",
     "the apex: twenty-four movements connected by a twelve-tone spine."),
    ("20", "nadir",              "Nadir",              14, "coda, de-resolution",
     "the coda: silence grows, pitch de-resolves; we arrive at the nadir."),
]


def main() -> int:
    ALBUMS_DIR.mkdir(parents=True, exist_ok=True)
    for n, slug, title, tracks, palette, narrative in ALBUMS:
        folder = ALBUMS_DIR / f"{n}_{slug}"
        folder.mkdir(exist_ok=True)
        manifest = folder / "MANIFEST.toml"
        liner = folder / "LINER.md"
        if not manifest.exists():
            manifest.write_text(textwrap.dedent(f"""\
                [album]
                n = {int(n)}
                slug = "{n}_{slug}"
                title = "{title}"
                planned_tracks = {tracks}

                [narrative]
                palette = "{palette}"
                arc = "{narrative}"

                [corpus]
                motifs = []
                """))
        if not liner.exists():
            liner.write_text(textwrap.dedent(f"""\
                # {title}

                _Album {int(n)} of 20 in the Nadir_Singleton corpus._

                {narrative}

                ## Motif palette
                {palette}

                ## Tracks (planned)
                _{tracks} tracks. Individual `manifest.toml` files will populate as we render._

                ## Production notes
                """))
    print(f"scaffolded {len(ALBUMS)} albums under {ALBUMS_DIR}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
