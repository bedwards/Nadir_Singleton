#!/usr/bin/env python3
"""Scaffold empty albums (07–19) per CORPUS.md plan.

Reads the top row count + palette from albums/CORPUS.md, then for each empty
album creates N track directories, each with:
- manifest.toml (key/scale/bpm/meter/bars/form/voice/section_repeat set to 8)
- lyric.txt (procedurally generated from the album's palette)
- NOTES.md (brief stub)

Deterministic: same script output every run. Only touches albums/NN_slug/
that currently have no NN_* subdirs.
"""
from __future__ import annotations

import hashlib
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ALBUMS = ROOT / "albums"

# (slug, num_tracks, palette_description) — extracted from CORPUS.md
PLAN = [
    ("07_praat_lieder", 16, "minimal art-song, Praat pitch-tier aware, intimate lieder"),
    ("08_mbrola_cabaret", 18, "ragtime / cabaret forms, MBROLA vaudeville swing"),
    ("09_formant_gardens", 14, "botanical ambient, formant gardens, still"),
    ("10_the_cent_maze", 16, "microtonal labyrinth, off-grid cent drift"),
    ("11_voicing_fraction", 18, "pop hooks, voicing fractions, earworm"),
    ("12_diphone_drift", 15, "tape-like slow change, diphone drift, warp"),
    ("13_egemaps_tarot", 22, "eGeMAPS feature-family archetypes, 22 cards"),
    ("14_silero_rooms", 16, "onset-driven percussive, VAD rooms"),
    ("15_fir_psalms", 14, "FIR-shaped hymns, csdr psalms"),
    ("16_pitch_pilgrimage", 16, "modal per track, pilgrimage through modes"),
    ("17_plosive_letters", 14, "consonant-forward, plosive letters"),
    ("18_dorian_weather", 16, "Dorian modal pop, weather as mode-change"),
    ("19_singleton_suite", 24, "long-form, 24 connected, singleton suite"),
]

KEYS = ["A", "B", "C", "D", "E", "F", "G"]
MINOR_SCALES = ["minor", "dorian", "phrygian", "locrian", "harmonic_minor"]
MAJOR_SCALES = ["major", "lydian", "mixolydian"]

# Small theme-tagged word pools — lyrics pick from these deterministically.
THEMES = {
    "07_praat_lieder": {
        "nouns": ["voice", "song", "line", "word", "syllable", "breath", "silence", "chord"],
        "verbs": ["sing", "hold", "turn", "rise", "fall", "listen", "wait", "answer"],
        "adjs": ["quiet", "slow", "warm", "near", "still", "bright", "dark", "close"],
    },
    "08_mbrola_cabaret": {
        "nouns": ["ragtime", "nightclub", "spotlight", "piano", "crowd", "diphone", "ivory", "shoe"],
        "verbs": ["swing", "stride", "tap", "bow", "turn", "laugh", "stroll", "encore"],
        "adjs": ["sharp", "crooked", "bright", "silver", "loud", "smoky", "quick", "fine"],
    },
    "09_formant_gardens": {
        "nouns": ["garden", "leaf", "root", "formant", "fern", "moss", "shade", "petal"],
        "verbs": ["grow", "bend", "breathe", "open", "fold", "wait", "sway", "rest"],
        "adjs": ["green", "wet", "cool", "soft", "deep", "slow", "old", "small"],
    },
    "10_the_cent_maze": {
        "nouns": ["cent", "maze", "wall", "turn", "drift", "step", "gap", "thread"],
        "verbs": ["bend", "drift", "slip", "lean", "find", "lose", "follow", "measure"],
        "adjs": ["narrow", "crooked", "thin", "off", "small", "near", "wrong", "true"],
    },
    "11_voicing_fraction": {
        "nouns": ["hook", "fraction", "hand", "line", "beat", "song", "chorus", "ear"],
        "verbs": ["sing", "catch", "hold", "carry", "return", "repeat", "linger", "echo"],
        "adjs": ["sweet", "bright", "loud", "warm", "short", "long", "clear", "new"],
    },
    "12_diphone_drift": {
        "nouns": ["tape", "drift", "reel", "head", "diphone", "hiss", "edge", "clock"],
        "verbs": ["slip", "warp", "melt", "pull", "thin", "stretch", "fold", "age"],
        "adjs": ["old", "slow", "worn", "thick", "loose", "warm", "soft", "far"],
    },
    "13_egemaps_tarot": {
        "nouns": ["card", "arch", "tower", "moon", "sun", "star", "hermit", "lover"],
        "verbs": ["turn", "shuffle", "deal", "read", "wait", "show", "hide", "name"],
        "adjs": ["hidden", "bright", "dark", "high", "low", "near", "far", "strange"],
    },
    "14_silero_rooms": {
        "nouns": ["room", "door", "wall", "floor", "window", "corner", "hallway", "onset"],
        "verbs": ["knock", "open", "close", "step", "turn", "wait", "cross", "land"],
        "adjs": ["empty", "wide", "small", "cold", "bright", "long", "quiet", "still"],
    },
    "15_fir_psalms": {
        "nouns": ["psalm", "filter", "hymn", "band", "voice", "song", "wing", "light"],
        "verbs": ["sing", "lift", "carry", "bless", "wait", "cross", "rise", "hold"],
        "adjs": ["high", "pure", "bright", "deep", "near", "far", "slow", "wide"],
    },
    "16_pitch_pilgrimage": {
        "nouns": ["road", "mode", "hill", "step", "pilgrim", "staff", "dust", "gate"],
        "verbs": ["walk", "climb", "cross", "turn", "rest", "carry", "return", "kneel"],
        "adjs": ["long", "old", "hard", "slow", "dry", "far", "low", "high"],
    },
    "17_plosive_letters": {
        "nouns": ["letter", "pop", "bell", "break", "kick", "pat", "top", "beat"],
        "verbs": ["tap", "hit", "drop", "pop", "strike", "land", "break", "stop"],
        "adjs": ["sharp", "hard", "quick", "short", "flat", "bright", "light", "close"],
    },
    "18_dorian_weather": {
        "nouns": ["cloud", "rain", "wind", "mode", "weather", "hill", "sky", "storm"],
        "verbs": ["turn", "fall", "drift", "lift", "bend", "soak", "wait", "clear"],
        "adjs": ["gray", "cool", "wide", "low", "warm", "long", "near", "soft"],
    },
    "19_singleton_suite": {
        "nouns": ["one", "voice", "suite", "thread", "line", "self", "end", "name"],
        "verbs": ["hold", "carry", "answer", "return", "fold", "name", "open", "end"],
        "adjs": ["single", "whole", "long", "final", "quiet", "near", "far", "true"],
    },
}

TITLES = {
    "07_praat_lieder": [
        "a small song", "the bright hold", "one long breath", "near the door",
        "afternoon lieder", "close to the mouth", "the quiet answer", "the slow turn",
        "a warm line", "held in two", "the still window", "the near reply",
        "by the lamp", "the old pitch", "the soft return", "the last song",
    ],
    "08_mbrola_cabaret": [
        "switchboard stomp", "stride for a dollar", "the ivory turn",
        "spotlight dime", "bright silver bow", "smoky encore",
        "velvet tap", "the quick stroll", "nightclub march", "ragtime refuse",
        "crooked cabaret", "late curtain", "laugh loud", "diphone waltz",
        "strut and tell", "the ivory return", "bow and bow", "tip the song",
    ],
    "09_formant_gardens": [
        "green formant", "wet ferns", "slow moss", "cool shade", "soft petal",
        "deep root", "the quiet open", "the fold of leaves", "bend low",
        "shade rest", "garden in the mouth", "small formant", "old petal",
        "breathe the garden",
    ],
    "10_the_cent_maze": [
        "narrow cent", "the crooked wall", "thin turn", "off by one",
        "small drift", "near the true", "wrong step", "follow the step",
        "measure the gap", "find the thread", "slip the maze",
        "crooked measure", "narrow drift", "thin true", "wrong wall", "lose the step",
    ],
    "11_voicing_fraction": [
        "sweet hook", "bright fraction", "short chorus", "clear beat",
        "long return", "warm linger", "the ear holds", "catch the line",
        "repeat the song", "new hand", "sing the hook", "carry the beat",
        "echo the song", "loud chorus", "short return", "clear linger",
        "the ear stays", "new line",
    ],
    "12_diphone_drift": [
        "old tape", "slow drift", "worn reel", "thick head", "loose hiss",
        "warm edge", "soft clock", "far fold", "stretch the tape",
        "age the drift", "warp the reel", "thin the head", "pull the hiss",
        "melt the edge", "slip the clock",
    ],
    "13_egemaps_tarot": [
        "the hidden card", "the bright arch", "the dark tower", "the high moon",
        "the low sun", "the near star", "the far hermit", "the strange lover",
        "the still death", "the fool", "the wheel", "the hermit returns",
        "the sun waits", "the moon names", "the star reads", "the tower falls",
        "the lover hides", "the hermit crosses", "the wheel turns",
        "the fool speaks", "the world holds", "the wheel names",
    ],
    "14_silero_rooms": [
        "empty room", "wide door", "small wall", "cold floor",
        "bright window", "long corner", "quiet hallway", "still onset",
        "knock and open", "close the door", "step the floor", "turn the corner",
        "wait in the hall", "cross the room", "land at the door", "the onset waits",
    ],
    "15_fir_psalms": [
        "high psalm", "pure filter", "bright hymn", "deep band",
        "near voice", "far song", "slow wing", "wide light",
        "sing the psalm", "lift the hymn", "carry the band", "bless the voice",
        "wait for song", "cross the light", "hold the hymn",
    ],
    "16_pitch_pilgrimage": [
        "long road", "old mode", "hard hill", "slow step",
        "dry pilgrim", "far staff", "low dust", "high gate",
        "walk the mode", "climb the hill", "cross the road", "turn the step",
        "rest at the gate", "carry the staff", "return the mode", "kneel the hill",
    ],
    "17_plosive_letters": [
        "sharp letter", "hard pop", "quick bell", "short break",
        "flat kick", "bright pat", "light top", "close beat",
        "tap the letter", "hit the bell", "drop the kick", "pop the break",
        "strike the top", "land the beat",
    ],
    "18_dorian_weather": [
        "gray cloud", "cool rain", "wide wind", "low mode",
        "warm weather", "long hill", "near sky", "soft storm",
        "turn the cloud", "fall the rain", "drift the wind", "lift the mode",
        "bend the weather", "soak the hill", "wait the sky", "clear the storm",
    ],
    "19_singleton_suite": [
        "single voice", "whole suite", "long thread", "final line",
        "quiet self", "near end", "far name", "true suite",
        "hold the voice", "carry the suite", "answer the thread",
        "return the line", "fold the self", "name the end", "open the name",
        "end the suite", "single line", "whole thread", "long self",
        "final voice", "quiet end", "near name", "far suite", "true voice",
    ],
}


def seed_for(slug: str, n: int) -> int:
    """Deterministic u64 seed from slug + track number."""
    h = hashlib.sha256(f"{slug}:{n}".encode()).digest()
    return int.from_bytes(h[:8], "big")


def pick(pool: list[str], seed: int, n: int) -> str:
    return pool[(seed >> (n * 3)) % len(pool)]


def gen_lyric(slug: str, n: int) -> str:
    theme = THEMES[slug]
    seed = seed_for(slug, n)
    nouns, verbs, adjs = theme["nouns"], theme["verbs"], theme["adjs"]
    lines = []
    # 12 lines of 2-4 words each, theme-locked.
    for i in range(12):
        s = seed + i * 0x9E3779B9
        pattern = (s >> 1) % 5
        if pattern == 0:
            lines.append(f"{pick(adjs, s, 0)} {pick(nouns, s, 1)}")
        elif pattern == 1:
            lines.append(f"{pick(verbs, s, 0)} the {pick(nouns, s, 1)}")
        elif pattern == 2:
            lines.append(f"the {pick(nouns, s, 0)} is {pick(adjs, s, 1)}")
        elif pattern == 3:
            lines.append(f"{pick(verbs, s, 0)} and {pick(verbs, s, 1)}")
        else:
            lines.append(f"{pick(adjs, s, 0)} and {pick(adjs, s, 1)} {pick(nouns, s, 2)}")
    return "\n".join(lines) + "\n"


def gen_manifest(slug: str, n: int, title: str) -> str:
    seed = seed_for(slug, n)
    key = KEYS[(seed >> 8) % 7]
    # Some albums pick minor-family modes, some major.
    minor_biased = slug in {
        "07_praat_lieder", "09_formant_gardens", "12_diphone_drift",
        "15_fir_psalms", "16_pitch_pilgrimage", "19_singleton_suite",
    }
    pool = MINOR_SCALES if minor_biased else MAJOR_SCALES + MINOR_SCALES
    scale = pool[(seed >> 16) % len(pool)]
    bpm = 60 + (seed >> 24) % 80   # 60–140
    bars = 16 + (seed >> 32) % 17  # 16–32
    return (
        f'[track]\n'
        f'n = {n}\n'
        f'slug = "{n:02d}_{slug_title(title)}"\n'
        f'title = "{title}"\n'
        f'key = "{key}"\n'
        f'scale = "{scale}"\n'
        f'bpm = {bpm}\n'
        f'meter = [4, 4]\n'
        f'bars = {bars}\n'
        f'form = "through-composed"\n'
        f'mbrola_voice = "us1"\n'
        f'section_repeat = 8\n'
        f'seed = {seed & 0x7FFFFFFF_FFFFFFFF}\n'
        f'\n'
        f'[targets]\n'
        f'tessitura_hz = [140, 440]\n'
        f'pitch_error_ceiling_cents = 20.0\n'
        f'vox_loudness_lufs = -18.0\n'
        f'\n'
        f'[dsp]\n'
        f'bed_presets = ["tonal_drone_triad", "shaped_noise_air"]\n'
    )


def slug_title(title: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", title.lower()).strip("_")


def main() -> int:
    made = 0
    for slug, n_tracks, palette in PLAN:
        album_dir = ALBUMS / slug
        # Only scaffold if empty of track dirs
        existing_tracks = [p for p in album_dir.iterdir() if p.is_dir()] if album_dir.exists() else []
        if existing_tracks:
            continue
        album_dir.mkdir(parents=True, exist_ok=True)
        # LINER.md
        liner = album_dir / "LINER.md"
        if not liner.exists():
            liner.write_text(f"# {slug.split('_', 1)[1].replace('_', ' ').title()}\n\n_palette:_ {palette}\n")
        titles = TITLES.get(slug, [f"track {i+1}" for i in range(n_tracks)])
        titles = (titles + [f"track {i+1}" for i in range(n_tracks)])[:n_tracks]
        for i, title in enumerate(titles, 1):
            tslug = f"{i:02d}_{slug_title(title)}"
            td = album_dir / tslug
            td.mkdir(exist_ok=True)
            (td / "manifest.toml").write_text(gen_manifest(slug, i, title))
            (td / "lyric.txt").write_text(gen_lyric(slug, i))
            (td / "NOTES.md").write_text(f"# {title}\n\n_album:_ {slug}\n_palette:_ {palette}\n")
            made += 1
        print(f"scaffolded {slug}: {n_tracks} tracks")
    print(f"total tracks created: {made}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
