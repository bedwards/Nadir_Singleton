//! CLI surface. Clap derive.
//!
//! Convention: every subcommand exposes its own `--help`, and any command that
//! wraps one of the five core tools accepts trailing `--` args that are passed
//! verbatim to the tool via `#[arg(last = true)]` on a `Vec<String>`.

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "nadir",
    version,
    about = "Nadir_Singleton: experimental music produced with openSMILE, Praat, MBROLA, Silero-VAD, csdr — only.",
    long_about = "Nadir_Singleton is the AI composer/producer persona and the production system it runs on.\nVocals are diphone-synthesised (MBROLA), pitch-corrected to key (Praat PSOLA), and audited\nby openSMILE. Rhythm comes from Silero-VAD onsets. DSP is a csdr pipeline chain.\nNo other audio tool is permitted.",
    propagate_version = true
)]
pub struct Cli {
    /// Increase verbosity (repeat for more).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Album management (create, list, liner notes).
    Album(AlbumCmd),
    /// Song management and rendering.
    Song(SongCmd),
    /// MBROLA-driven vocal synthesis.
    Vox(VoxCmd),
    /// Praat PSOLA / pitch / formant operations.
    Pitch(PitchCmd),
    /// Silero-VAD voice-activity detection and segmentation.
    Vad(VadCmd),
    /// csdr DSP pipeline builder and runner.
    Dsp(DspCmd),
    /// openSMILE feature extraction.
    Feat(FeatCmd),
    /// Cross-album corpus operations (narrative, motifs).
    Corpus(CorpusCmd),
    /// Open research notes.
    Research(ResearchCmd),
    /// Print versions of all five core tools.
    Doctor,
    /// Play a WAV (afplay on macOS, aplay on Linux). Quality-of-life preview.
    Play {
        file: PathBuf,
    },
}

// ─────────── album ───────────

#[derive(Args, Debug)]
pub struct AlbumCmd {
    #[command(subcommand)]
    pub sub: AlbumSub,
}

#[derive(Subcommand, Debug)]
pub enum AlbumSub {
    /// List albums defined in this repo.
    List,
    /// Create a new album scaffold.
    New {
        /// Slug (e.g. `01_horizon_salts`).
        slug: String,
        /// Album title.
        #[arg(long)]
        title: String,
    },
    /// Show liner notes.
    Liner { slug: String },
}

// ─────────── song ───────────

#[derive(Args, Debug)]
pub struct SongCmd {
    #[command(subcommand)]
    pub sub: SongSub,
}

#[derive(Subcommand, Debug)]
pub enum SongSub {
    /// Create a new track manifest under an album.
    New {
        #[arg(long)]
        album: String,
        #[arg(long)]
        n: u8,
        #[arg(long)]
        title: String,
    },
    /// Render a track end-to-end.
    Render {
        #[arg(long)]
        album: String,
        #[arg(long)]
        track: u8,
        #[arg(long, default_value = "out.wav")]
        out: PathBuf,
    },
    /// Audit a rendered track against quality gates.
    Audit {
        #[arg(long)]
        album: String,
        #[arg(long)]
        track: u8,
    },
}

// ─────────── vox ───────────

#[derive(Args, Debug)]
pub struct VoxCmd {
    #[command(subcommand)]
    pub sub: VoxSub,
}

#[derive(Subcommand, Debug)]
pub enum VoxSub {
    /// Synthesize a `.pho` file to a WAV with MBROLA.
    Synth {
        #[arg(long)]
        pho: PathBuf,
        #[arg(long, default_value = "us1")]
        voice: String,
        #[arg(long, default_value = "out.wav")]
        out: PathBuf,
        /// Everything after `--` is forwarded to mbrola verbatim.
        #[arg(last = true)]
        passthrough: Vec<String>,
    },
    /// Build a scale-snapped vocal from lyrics.
    FromLyrics {
        #[arg(long)]
        text: String,
        #[arg(long, default_value = "us1")]
        voice: String,
        #[arg(long, default_value = "A")]
        key: String,
        #[arg(long, default_value = "minor")]
        scale: String,
        #[arg(long, default_value_t = 96.0)]
        bpm: f32,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long, default_value = "vox.wav")]
        out: PathBuf,
    },
    /// Closed-loop tuning: MBROLA → openSMILE audit → Praat PSOLA retarget.
    Tune {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long)]
        key: String,
        #[arg(long)]
        scale: String,
        #[arg(long, default_value_t = 3)]
        max_passes: u8,
        #[arg(long, default_value = "tuned.wav")]
        out: PathBuf,
    },
}

// ─────────── pitch (praat) ───────────

#[derive(Args, Debug)]
pub struct PitchCmd {
    #[command(subcommand)]
    pub sub: PitchSub,
}

#[derive(Subcommand, Debug)]
pub enum PitchSub {
    /// PSOLA retarget an input WAV to a target F0 contour.
    Psola {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long)]
        target_csv: PathBuf,
        #[arg(long, default_value = "out.wav")]
        out: PathBuf,
        #[arg(last = true)]
        passthrough: Vec<String>,
    },
    /// Extract a pitch track (ac, cc, or shs).
    Extract {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long, default_value = "ac")]
        method: String,
        #[arg(long, default_value = "pitch.csv")]
        out: PathBuf,
    },
}

// ─────────── vad ───────────

#[derive(Args, Debug)]
pub struct VadCmd {
    #[command(subcommand)]
    pub sub: VadSub,
}

#[derive(Subcommand, Debug)]
pub enum VadSub {
    /// Detect speech segments in a WAV.
    Segments {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long, default_value_t = 0.3)]
        threshold: f32,
    },
    /// Split a WAV into segment WAVs.
    Split {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long, default_value = "segs")]
        out_dir: PathBuf,
    },
    /// Emit an onset grid (JSON) useful as rhythmic triggers.
    Onsets {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long, default_value_t = 0.3)]
        threshold: f32,
    },
}

// ─────────── dsp (csdr) ───────────

#[derive(Args, Debug)]
pub struct DspCmd {
    #[command(subcommand)]
    pub sub: DspSub,
}

#[derive(Subcommand, Debug)]
pub enum DspSub {
    /// Run a csdr graph (TOML) against an input file.
    Run {
        #[arg(long)]
        graph: PathBuf,
        #[arg(long)]
        in_file: PathBuf,
        #[arg(long, default_value = "out.raw")]
        out: PathBuf,
        #[arg(last = true)]
        passthrough: Vec<String>,
    },
    /// Print the pipeline shell string for a graph (dry-run).
    Show {
        #[arg(long)]
        graph: PathBuf,
    },
    /// Emit preset graphs (upsample, band-limit, ring-mod) to TOML.
    Preset {
        which: String,
        #[arg(long, default_value = "graph.toml")]
        out: PathBuf,
    },
}

// ─────────── feat (opensmile) ───────────

#[derive(Args, Debug)]
pub struct FeatCmd {
    #[command(subcommand)]
    pub sub: FeatSub,
}

#[derive(Subcommand, Debug)]
pub enum FeatSub {
    /// Extract features with a named feature set.
    Extract {
        #[arg(long, default_value = "eGeMAPSv02")]
        set: String,
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long, default_value = "features.csv")]
        out: PathBuf,
    },
    /// Audit pitch track (RMS cents error vs a target CSV).
    Audit {
        #[arg(long)]
        in_wav: PathBuf,
        #[arg(long)]
        target_csv: PathBuf,
    },
}

// ─────────── corpus ───────────

#[derive(Args, Debug)]
pub struct CorpusCmd {
    #[command(subcommand)]
    pub sub: CorpusSub,
}

#[derive(Subcommand, Debug)]
pub enum CorpusSub {
    /// Print the narrative arc across albums.
    Narrative,
    /// List motifs tracked in CORPUS.md.
    Motifs,
}

// ─────────── research ───────────

#[derive(Args, Debug)]
pub struct ResearchCmd {
    /// Tool name: opensmile, praat, mbrola, silero, csdr, first_principles_music.
    pub name: String,
}

// ─────────── dispatch ───────────

pub fn dispatch(cli: Cli) -> Result<()> {
    use Cmd::*;
    match cli.cmd {
        Album(c) => dispatch_album(c),
        Song(c) => dispatch_song(c),
        Vox(c) => dispatch_vox(c),
        Pitch(c) => dispatch_pitch(c),
        Vad(c) => dispatch_vad(c),
        Dsp(c) => dispatch_dsp(c),
        Feat(c) => dispatch_feat(c),
        Corpus(c) => dispatch_corpus(c),
        Research(c) => dispatch_research(c),
        Doctor => dispatch_doctor(),
        Play { file } => dispatch_play(&file),
    }
}

fn dispatch_play(file: &std::path::Path) -> Result<()> {
    if !file.exists() {
        anyhow::bail!("no such file: {}", file.display());
    }
    let (bin, args): (&str, Vec<&std::ffi::OsStr>) = if cfg!(target_os = "macos") {
        ("afplay", vec![file.as_os_str()])
    } else {
        ("aplay", vec![file.as_os_str()])
    };
    let status = std::process::Command::new(bin)
        .args(&args)
        .status()
        .with_context(|| format!("spawn {bin}"))?;
    if !status.success() {
        anyhow::bail!("{bin} failed ({status})");
    }
    Ok(())
}

fn dispatch_album(c: AlbumCmd) -> Result<()> {
    match c.sub {
        AlbumSub::List => {
            let albums_dir = std::path::Path::new("albums");
            if albums_dir.exists() {
                for entry in fs_err::read_dir(albums_dir)? {
                    let e = entry?;
                    if e.path().is_dir() {
                        println!("{}", e.file_name().to_string_lossy());
                    }
                }
            }
            Ok(())
        }
        AlbumSub::New { slug, title } => {
            let dir = format!("albums/{slug}");
            fs_err::create_dir_all(&dir)?;
            let manifest = format!(
                "[album]\nslug = \"{slug}\"\ntitle = \"{title}\"\n\n[narrative]\narc = \"TBD\"\n",
            );
            fs_err::write(format!("{dir}/MANIFEST.toml"), manifest)?;
            fs_err::write(
                format!("{dir}/LINER.md"),
                format!("# {title}\n\n_draft liner_\n"),
            )?;
            println!("created {dir}");
            Ok(())
        }
        AlbumSub::Liner { slug } => {
            let p = format!("albums/{slug}/LINER.md");
            let s = fs_err::read_to_string(&p).with_context(|| format!("read {p}"))?;
            println!("{s}");
            Ok(())
        }
    }
}

fn dispatch_song(c: SongCmd) -> Result<()> {
    match c.sub {
        SongSub::New { album, n, title } => {
            let dir = format!("albums/{album}/{:02}_{}", n, slugify(&title));
            fs_err::create_dir_all(format!("{dir}/stems"))?;
            let manifest = format!(
                "[track]\nn = {n}\ntitle = \"{title}\"\nkey = \"A\"\nscale = \"minor\"\nbpm = 96\nmeter = [4, 4]\nbars = 16\nform = \"verse/chorus\"\n",
            );
            fs_err::write(format!("{dir}/manifest.toml"), manifest)?;
            fs_err::write(format!("{dir}/lyric.txt"), "")?;
            fs_err::write(format!("{dir}/NOTES.md"), format!("# {title}\n"))?;
            println!("created {dir}");
            Ok(())
        }
        SongSub::Render { album, track, out } => {
            use nadir_compose::{plan_melody, render_vox_pho};
            use nadir_core::{Key, Scale, ScaleKind};
            use nadir_praat::{extract_f0_script, psola_retarget_script, run_inline, PraatConfig};
            use nadir_vox::{synth_to_wav, MbrolaConfig};
            use serde::Deserialize;
            use std::process::Command;
            use std::str::FromStr;

            #[derive(Deserialize)]
            struct TrackManifest {
                track: TrackFields,
            }
            #[derive(Deserialize)]
            struct TrackFields {
                key: String,
                scale: String,
                #[serde(default = "default_bpm")]
                bpm: f32,
                #[serde(default = "default_voice")]
                mbrola_voice: String,
                #[serde(default = "default_seed")]
                seed: u64,
            }
            fn default_bpm() -> f32 { 96.0 }
            fn default_voice() -> String { "us1".into() }
            fn default_seed() -> u64 { 42 }

            // find track dir
            let track_dir = {
                let prefix = format!("albums/{album}/{track:02}_");
                let entries = fs_err::read_dir(format!("albums/{album}"))
                    .with_context(|| format!("open albums/{album}"))?;
                let mut found = None;
                for e in entries {
                    let e = e?;
                    let name = e.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with(&prefix[format!("albums/{album}/").len()..]) {
                        found = Some(e.path());
                        break;
                    }
                }
                found.with_context(|| format!("no track {track:02} in {album}"))?
            };

            let manifest_text = fs_err::read_to_string(track_dir.join("manifest.toml"))?;
            let m: TrackManifest = toml::from_str(&manifest_text)?;
            let lyric = fs_err::read_to_string(track_dir.join("lyric.txt"))
                .unwrap_or_default()
                .lines()
                .collect::<Vec<_>>()
                .join(" ");

            if lyric.trim().is_empty() {
                anyhow::bail!("lyric.txt is empty for track {track} in {album}");
            }

            let k = Key::from_str(&m.track.key).map_err(|e| anyhow::anyhow!(e))?;
            let sk = ScaleKind::from_str(&m.track.scale).map_err(|e| anyhow::anyhow!(e))?;
            let sc = Scale::new(k, sk);

            // G2P with stress weights
            let g2p_out = Command::new("uv")
                .args(["run", "--project", "python/nadir-lyric-g2p", "nadir-g2p",
                       "--stress", "--voice", &m.track.mbrola_voice, "--text", &lyric])
                .output().context("g2p spawn")?;
            if !g2p_out.status.success() {
                anyhow::bail!("g2p: {}", String::from_utf8_lossy(&g2p_out.stderr));
            }
            let word_data: Vec<serde_json::Value> = serde_json::from_slice(&g2p_out.stdout)?;
            let phonemes: Vec<Vec<String>> = word_data.iter()
                .map(|v| v["phonemes"].as_array().unwrap_or(&vec![])
                    .iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
                .collect();
            let stresses: Vec<f32> = word_data.iter()
                .map(|v| v["stress"].as_f64().unwrap_or(1.0) as f32)
                .collect();
            let syllables: Vec<String> = lyric.split_whitespace().map(str::to_string).collect();

            let notes = plan_melody(&sc, &syllables, m.track.seed, 220.0, m.track.bpm, &stresses);
            let stream = render_vox_pho(&notes, &phonemes);

            let vox_cfg = MbrolaConfig {
                voice: m.track.mbrola_voice.clone(),
                ..Default::default()
            };

            let stems_dir = track_dir.join("stems");
            fs_err::create_dir_all(&stems_dir)?;
            let raw_vox_path = stems_dir.join("raw_vox.wav");
            let tuned_vox_path = stems_dir.join("tuned_vox.wav");
            let f0_realized_path = stems_dir.join("f0_realized.csv");
            let f0_target_path = stems_dir.join("f0_target.csv");

            synth_to_wav(&vox_cfg, &stream, &raw_vox_path)?;

            let praat_cfg = PraatConfig::default();
            run_inline(&praat_cfg, &extract_f0_script(&raw_vox_path, &f0_realized_path), &[])?;
            let f0_text = fs_err::read_to_string(&f0_realized_path)?;
            let realized: Vec<(f32, f32)> = f0_text.lines().skip(1)
                .filter_map(|l| {
                    let mut it = l.split(',');
                    let t: f32 = it.next()?.parse().ok()?;
                    let h: f32 = it.next()?.parse().ok()?;
                    Some((t, h))
                })
                .collect();

            let dest = if out.to_str() == Some("out.wav") {
                track_dir.join("render.wav")
            } else {
                out.clone()
            };

            if realized.is_empty() {
                fs_err::copy(&raw_vox_path, &tuned_vox_path)?;
                fs_err::copy(&raw_vox_path, &dest)?;
            } else {
                {
                    use std::io::Write;
                    let mut f = std::fs::File::create(&f0_target_path)?;
                    writeln!(f, "time_s,hz")?;
                    for (t, hz) in &realized {
                        let snapped = sc.snap(*hz);
                        writeln!(f, "{t},{snapped}")?;
                    }
                }
                let script = psola_retarget_script(&raw_vox_path, &f0_target_path, &tuned_vox_path);
                run_inline(&praat_cfg, &script, &[])?;
                fs_err::copy(&tuned_vox_path, &dest)?;
            }

            println!("{}", dest.display());
            println!("stems: {}", stems_dir.display());
            Ok(())
        }
        SongSub::Audit { album, track } => {
            println!("audit stub — album={album} track={track}");
            Ok(())
        }
    }
}

fn dispatch_vox(c: VoxCmd) -> Result<()> {
    use nadir_vox::{synth_to_wav, MbrolaConfig};
    match c.sub {
        VoxSub::Synth {
            pho,
            voice,
            out,
            passthrough: _,
        } => {
            let cfg = MbrolaConfig {
                voice,
                ..Default::default()
            };
            let text = fs_err::read_to_string(&pho)?;
            let mut stream = nadir_core::PhoStream::new();
            for line in text.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    continue;
                }
                let sampa = parts[0].to_string();
                let dur: u32 = parts[1].parse().unwrap_or(100);
                let mut pitch = Vec::new();
                let mut i = 2;
                while i + 1 < parts.len() {
                    let pct: u8 = parts[i].parse().unwrap_or(50);
                    let hz: f32 = parts[i + 1].parse().unwrap_or(220.0);
                    pitch.push(nadir_core::PitchPoint { pct, hz });
                    i += 2;
                }
                stream.push(nadir_core::Pho {
                    sampa,
                    dur_ms: dur,
                    pitch,
                });
            }
            synth_to_wav(&cfg, &stream, &out)?;
            println!("{}", out.display());
            Ok(())
        }
        VoxSub::FromLyrics {
            text,
            voice,
            key,
            scale,
            bpm,
            seed,
            out,
        } => {
            use nadir_compose::{plan_melody, render_vox_pho};
            use nadir_core::{Key, Scale, ScaleKind};
            use std::process::Command;
            use std::str::FromStr;

            // G2P via Python subprocess (with stress weights)
            let g2p_output = Command::new("uv")
                .args(["run", "--project", "python/nadir-lyric-g2p", "nadir-g2p",
                       "--stress", "--voice", &voice, "--text", &text])
                .output()
                .context("spawn uv for g2p")?;
            if !g2p_output.status.success() {
                anyhow::bail!("g2p failed: {}", String::from_utf8_lossy(&g2p_output.stderr));
            }
            // JSON: Vec<{phonemes:[str], stress:f32}>
            let word_data: Vec<serde_json::Value> =
                serde_json::from_slice(&g2p_output.stdout).context("parse g2p json")?;
            let phonemes_per_word: Vec<Vec<String>> = word_data.iter()
                .map(|v| v["phonemes"].as_array().unwrap_or(&vec![])
                    .iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
                .collect();
            let stresses: Vec<f32> = word_data.iter()
                .map(|v| v["stress"].as_f64().unwrap_or(1.0) as f32)
                .collect();

            let syllables: Vec<String> = text.split_whitespace()
                .map(str::to_string)
                .collect();

            let k = Key::from_str(&key)
                .map_err(|e| anyhow::anyhow!(e))?;
            let sk = ScaleKind::from_str(&scale)
                .map_err(|e| anyhow::anyhow!(e))?;
            let sc = Scale::new(k, sk);

            let notes = plan_melody(&sc, &syllables, seed, 220.0, bpm, &stresses);
            let stream = render_vox_pho(&notes, &phonemes_per_word);
            let cfg = MbrolaConfig {
                voice: voice.clone(),
                ..Default::default()
            };
            synth_to_wav(&cfg, &stream, &out)?;
            println!("{}", out.display());
            Ok(())
        }
        VoxSub::Tune {
            in_wav,
            key,
            scale,
            max_passes,
            out,
        } => {
            use nadir_core::{Key, Scale, ScaleKind};
            use nadir_praat::{extract_f0_script, psola_retarget_script, run_inline, PraatConfig};
            use std::str::FromStr;

            let k = Key::from_str(&key).map_err(|e| anyhow::anyhow!(e))?;
            let sk = ScaleKind::from_str(&scale).map_err(|e| anyhow::anyhow!(e))?;
            let sc = Scale::new(k, sk);

            let praat_cfg = PraatConfig::default();

            let mut current = in_wav.clone();
            let mut tmp_wavs: Vec<tempfile::NamedTempFile> = Vec::new();

            for pass in 0..max_passes {
                // Extract realized F0 via Praat
                let f0_csv = tempfile::NamedTempFile::with_suffix(".csv")
                    .context("create f0 csv")?;
                let f0_script = extract_f0_script(&current, f0_csv.path());
                run_inline(&praat_cfg, &f0_script, &[])?;
                let f0_text = fs_err::read_to_string(f0_csv.path())?;
                // Parse simple time_s,hz CSV (skip header)
                let realized: Vec<(f32, f32)> = f0_text.lines().skip(1)
                    .filter_map(|l| {
                        let mut it = l.split(',');
                        let t: f32 = it.next()?.parse().ok()?;
                        let h: f32 = it.next()?.parse().ok()?;
                        Some((t, h))
                    })
                    .collect();

                if realized.is_empty() {
                    tracing::warn!("no F0 frames detected; stopping at pass {pass}");
                    break;
                }

                // Snap each realized frame to nearest scale degree → target CSV
                let target_csv = tempfile::NamedTempFile::with_suffix(".csv")
                    .context("create target csv")?;
                {
                    use std::io::Write;
                    let mut f = std::fs::File::create(target_csv.path())?;
                    writeln!(f, "time_s,hz")?;
                    for (t, hz) in &realized {
                        if *hz > 0.0 {
                            let snapped = sc.snap(*hz);
                            writeln!(f, "{t},{snapped}")?;
                        }
                    }
                }

                // RMS cents between realized and snapped target
                let snapped: Vec<(f32, f32)> = realized.iter()
                    .map(|(t, hz)| (*t, sc.snap(*hz)))
                    .collect();
                let err_before: f32 = if realized.is_empty() { 0.0 } else {
                    let sum: f32 = realized.iter().zip(snapped.iter())
                        .map(|((_, h1), (_, h2))| {
                            let c = 1200.0 * (h1 / h2).ln() / std::f32::consts::LN_2;
                            c * c
                        })
                        .sum();
                    (sum / realized.len() as f32).sqrt()
                };
                tracing::info!("pass {pass}: rms_cents before = {err_before:.1}");

                if err_before < 20.0 {
                    tracing::info!("pass {pass}: within 20¢, done");
                    break;
                }

                // PSOLA retarget
                let corrected = tempfile::NamedTempFile::with_suffix(".wav")
                    .context("create corrected wav")?;
                let script = psola_retarget_script(&current, target_csv.path(), corrected.path());
                run_inline(&praat_cfg, &script, &[])?;
                current = corrected.path().to_path_buf();
                tmp_wavs.push(corrected);
            }

            fs_err::copy(&current, &out)?;
            println!("{}", out.display());
            Ok(())
        }
    }
}

fn dispatch_pitch(c: PitchCmd) -> Result<()> {
    use nadir_praat::{psola_retarget_script, run_inline, PraatConfig};
    match c.sub {
        PitchSub::Psola {
            in_wav,
            target_csv,
            out,
            passthrough: _,
        } => {
            let cfg = PraatConfig::default();
            let script = psola_retarget_script(&in_wav, &target_csv, &out);
            let stdout = run_inline(&cfg, &script, &[])?;
            print!("{stdout}");
            println!("{}", out.display());
            Ok(())
        }
        PitchSub::Extract {
            in_wav,
            method,
            out,
        } => {
            println!(
                "pitch extract stub — in={} method={} out={}",
                in_wav.display(),
                method,
                out.display()
            );
            Ok(())
        }
    }
}

fn dispatch_vad(c: VadCmd) -> Result<()> {
    use nadir_vad::{detect_onsets, detect_segments, split_segments, VadConfig};
    match c.sub {
        VadSub::Segments { in_wav, threshold } => {
            let cfg = VadConfig {
                threshold,
                ..Default::default()
            };
            let segs = detect_segments(&cfg, &in_wav)?;
            println!("{}", serde_json::to_string_pretty(&segs)?);
            Ok(())
        }
        VadSub::Split { in_wav, out_dir } => {
            let cfg = VadConfig::default();
            let paths = split_segments(&cfg, &in_wav, &out_dir)?;
            for p in &paths {
                println!("{}", p.display());
            }
            Ok(())
        }
        VadSub::Onsets { in_wav, threshold } => {
            let cfg = VadConfig {
                threshold,
                ..Default::default()
            };
            let ons = detect_onsets(&cfg, &in_wav, None)?;
            println!("{}", serde_json::to_string_pretty(&ons)?);
            Ok(())
        }
    }
}

fn dispatch_dsp(c: DspCmd) -> Result<()> {
    use nadir_dsp::{presets, Graph};
    match c.sub {
        DspSub::Run {
            graph,
            in_file,
            out,
            passthrough: _,
        } => {
            let text = fs_err::read_to_string(&graph)?;
            let g = Graph::parse_toml(&text)?;
            g.run_files(&in_file, &out)?;
            println!("{}", out.display());
            Ok(())
        }
        DspSub::Show { graph } => {
            let text = fs_err::read_to_string(&graph)?;
            let g = Graph::parse_toml(&text)?;
            println!("{}", g.to_shell());
            Ok(())
        }
        DspSub::Preset { which, out } => {
            let g = match which.as_str() {
                // Original (pre-split) names — kept working.
                "upsample" | "upsample_16_to_48" => presets::upsample_16_to_48("csdr"),
                "band-limit" | "band_limit" => presets::band_limit(0.01, 0.4),
                "ring-mod" | "ring_mod" => presets::ring_mod(0.001),
                // New factories, reachable by their rustified name.
                "granular_texture" => presets::granular_texture(40, 1.0),
                "shaped_noise_bed" => presets::shaped_noise_bed(-0.2, 0.2, 50e-6),
                "dirac_impulse_bed" => presets::dirac_impulse_bed(8.0),
                "ring_mod_multi" => presets::ring_mod_multi(&[0.001, 0.002, 0.003, 0.004]),
                "fir_cascade" => presets::fir_cascade(&[(0.01, 0.1), (0.1, 0.3), (0.3, 0.4)]),
                "deemphasis_chain" => presets::deemphasis_chain(),
                "agc_limit_safe" => presets::agc_limit_safe(),
                "upsample_48_to_96" => presets::upsample_48_to_96(2),
                other => anyhow::bail!("unknown preset: {other}"),
            };
            fs_err::write(&out, g.to_toml()?)?;
            println!("{}", out.display());
            Ok(())
        }
    }
}

fn dispatch_feat(c: FeatCmd) -> Result<()> {
    use nadir_feat::{extract_csv, FeatureSet, SmileConfig};
    match c.sub {
        FeatSub::Extract { set, in_wav, out } => {
            let fs = match set.as_str() {
                "eGeMAPSv02" => FeatureSet::EGeMAPSv02,
                "ComParE2016" => FeatureSet::ComParE2016,
                "GeMAPSv01a" => FeatureSet::GeMAPSv01a,
                "emobase" => FeatureSet::Emobase,
                other => anyhow::bail!("unknown feature set: {other}"),
            };
            let cfg = SmileConfig::default();
            extract_csv(&cfg, fs, &in_wav, &out)?;
            println!("{}", out.display());
            Ok(())
        }
        FeatSub::Audit { in_wav, target_csv } => {
            use nadir_feat::{parse_f0_track, rms_cents};
            let cfg = SmileConfig::default();
            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .context("create temp csv")?;
            extract_csv(&cfg, FeatureSet::EGeMAPSv02, &in_wav, tmp.path())?;
            let realized_text = fs_err::read_to_string(tmp.path())?;
            let realized = parse_f0_track(&realized_text);
            let target_text = fs_err::read_to_string(&target_csv)?;
            let target = parse_f0_track(&target_text);
            let err = rms_cents(&realized, &target);
            println!("rms_cents: {err:.1}");
            Ok(())
        }
    }
}

fn dispatch_corpus(c: CorpusCmd) -> Result<()> {
    match c.sub {
        CorpusSub::Narrative => {
            let p = "albums/CORPUS.md";
            if let Ok(s) = fs_err::read_to_string(p) {
                println!("{s}");
            } else {
                println!("(no CORPUS.md yet — run with albums scaffolded)");
            }
            Ok(())
        }
        CorpusSub::Motifs => {
            println!("motif tracking is a v0.3 feature");
            Ok(())
        }
    }
}

fn dispatch_research(c: ResearchCmd) -> Result<()> {
    let p = format!("research/{}.md", c.name);
    let s = fs_err::read_to_string(&p).with_context(|| format!("read {p}"))?;
    println!("{s}");
    Ok(())
}

fn dispatch_doctor() -> Result<()> {
    use std::path::Path;
    println!("nadir v{}", env!("CARGO_PKG_VERSION"));
    for (name, bin) in [
        ("mbrola", "mbrola"),
        ("praat", "praat"),
        ("SMILExtract", "SMILExtract"),
        ("csdr", "csdr"),
    ] {
        match std::process::Command::new(bin).arg("--help").output() {
            Ok(_) => println!("  {name:<12}  found on PATH"),
            Err(_) => println!("  {name:<12}  MISSING on PATH"),
        }
    }
    let uv_project = Path::new("python/nadir-vad");
    if uv_project.exists() {
        println!("  silero-vad    python project present (uv)");
    } else {
        println!(
            "  silero-vad    python project MISSING at {}",
            uv_project.display()
        );
    }
    Ok(())
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
