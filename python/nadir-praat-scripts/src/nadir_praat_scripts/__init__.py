"""Praat script library (strings, paths, and small generators).

The Rust `nadir-praat` crate can either embed these scripts via build.rs or call
this package through subprocess during dev iteration.
"""
from importlib.resources import files

SCRIPTS = {
    "psola_retarget": "psola_retarget.praat",
    "pitch_extract": "pitch_extract.praat",
    "formant_shift": "formant_shift.praat",
    "duration_warp": "duration_warp.praat",
    "klatt_texture": "klatt_texture.praat",
}


def script_path(name: str) -> str:
    p = files("nadir_praat_scripts").joinpath("scripts", SCRIPTS[name])
    return str(p)


def script_body(name: str) -> str:
    return files("nadir_praat_scripts").joinpath("scripts", SCRIPTS[name]).read_text()
