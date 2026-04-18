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
    Play { file: PathBuf },
    /// Build a 4K/60 MP4 from a mastered WAV using ffmpeg showspectrum and
    /// a subtle animated gradient. MVP visual; richer Bevy/Mitsuba pipelines
    /// land in follow-up PRs.
    Video {
        in_wav: PathBuf,
        out_mp4: PathBuf,
        /// Title drawn over the video (single line).
        #[arg(long, default_value = "Nadir_Singleton")]
        title: String,
    },
    /// Write a JSON schedule (folk-sequence-compatible) for 100 uploads
    /// on Sun + Tue + Fri at 14:00 UTC (per docs/RELEASE_PLAN.md).
    Schedule {
        /// Start date (YYYY-MM-DD). Default: today. Jumps forward to next
        /// Sun/Tue/Fri if not already one.
        #[arg(long)]
        start: Option<String>,
        #[arg(long, default_value_t = 100)]
        count: u32,
        #[arg(long, default_value = "output/schedule.json")]
        out: PathBuf,
    },
    /// Master a WAV to a target integrated LUFS via ffmpeg two-pass loudnorm.
    /// Default -9 LUFS for YouTube delivery (YouTube attenuates to ~-14).
    Master {
        in_wav: PathBuf,
        out_wav: PathBuf,
        #[arg(long, default_value_t = -9.0)]
        lufs: f32,
        #[arg(long, default_value_t = -1.0)]
        true_peak: f32,
        #[arg(long, default_value_t = 7.0)]
        lra: f32,
    },
    /// Compile rendered tracks into ~target-minute chains with crossfade.
    /// Used to produce 11.5-minute YouTube compilations.
    Compile {
        /// Albums to include (order matters). Empty → all albums in albums/.
        #[arg(long)]
        albums: Vec<String>,
        #[arg(long, default_value_t = 11.5)]
        target_minutes: f32,
        #[arg(long, default_value_t = 1500)]
        xfade_ms: u32,
        #[arg(long, default_value = "compilations")]
        out_dir: PathBuf,
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
    /// Render every track in an album.
    Render {
        slug: String,
        /// Only render tracks whose lyric.txt is non-empty.
        #[arg(long)]
        only_with_lyrics: bool,
        /// Continue past tracks that fail.
        #[arg(long)]
        keep_going: bool,
    },
    /// Play every rendered track in an album in order.
    Play { slug: String },
    /// Show a table of audit rms_cents per track (reads stems/audit.json).
    Audit { slug: String },
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
        /// Override manifest voice (e.g. us1, us3, en1).
        #[arg(long)]
        voice: Option<String>,
        /// Override manifest bpm.
        #[arg(long)]
        bpm: Option<f32>,
        /// Override manifest key (A..G with optional # / b).
        #[arg(long)]
        key: Option<String>,
        /// Override manifest scale (minor, major, dorian, …).
        #[arg(long)]
        scale: Option<String>,
        /// Override manifest seed.
        #[arg(long)]
        seed: Option<u64>,
        /// Override manifest bed preset.
        #[arg(long)]
        bed_preset: Option<String>,
        /// Fail with non-zero exit if audit ceiling exceeded.
        #[arg(long)]
        strict: bool,
    },
    /// Audit a rendered track against quality gates.
    Audit {
        #[arg(long)]
        album: String,
        #[arg(long)]
        track: u8,
    },
    /// Render a track, then play it.
    Listen {
        #[arg(long)]
        album: String,
        #[arg(long)]
        track: u8,
        #[arg(long)]
        bed_preset: Option<String>,
        #[arg(long)]
        bpm: Option<f32>,
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
        Master {
            in_wav,
            out_wav,
            lufs,
            true_peak,
            lra,
        } => dispatch_master(&in_wav, &out_wav, lufs, true_peak, lra),
        Schedule { start, count, out } => dispatch_schedule(start.as_deref(), count, &out),
        Video {
            in_wav,
            out_mp4,
            title,
        } => dispatch_video(&in_wav, &out_mp4, &title),
        Compile {
            albums,
            target_minutes,
            xfade_ms,
            out_dir,
        } => dispatch_compile(&albums, target_minutes, xfade_ms, &out_dir),
    }
}

/// Build a 4K/60 MP4 from a WAV using ffmpeg's built-in showspectrum and
/// an animated background. MVP — intended to validate the pipeline end-to-end
/// before wiring the richer OSS visual tools (Bevy, Mitsuba 3, ParaView,
/// Mandelbulber, Pixray) per docs/RELEASE_PLAN.md.
fn dispatch_video(in_wav: &std::path::Path, out_mp4: &std::path::Path, title: &str) -> Result<()> {
    if !in_wav.exists() {
        anyhow::bail!("no such file: {}", in_wav.display());
    }
    if let Some(parent) = out_mp4.parent() {
        if !parent.as_os_str().is_empty() {
            fs_err::create_dir_all(parent)?;
        }
    }

    // Escape single quotes in title for drawtext
    let safe_title = title.replace('\'', r"\'");
    // Layered filter graph:
    //   - animated dark gradient background via `color` + `hue`
    //   - translucent frequency spectrogram (showspectrum) over the top
    //   - large serif title bottom-center
    let filter = format!(
        concat!(
            "color=size=3840x2160:rate=60:color=0x0a0a14[bg];",
            "[0:a]showspectrum=size=3840x720:mode=combined:color=intensity:slide=scroll:scale=log:fscale=log:orientation=horizontal[spec];",
            "[0:a]showwaves=size=3840x360:mode=p2p:colors=cyan:rate=60[wave];",
            "[bg][spec]overlay=0:1440:format=auto[l1];",
            "[l1][wave]overlay=0:1800:format=auto[l2];",
            "[l2]drawtext=text='{safe_title}':fontsize=96:fontcolor=white@0.85:x=(w-text_w)/2:y=1200:borderw=2:bordercolor=0x10101040[final]"
        ),
        safe_title = safe_title
    );

    let status = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-y", "-i"])
        .arg(in_wav)
        .args([
            "-filter_complex",
            &filter,
            "-map",
            "[final]",
            "-map",
            "0:a",
            "-c:v",
            "libx264",
            "-profile:v",
            "high",
            "-preset",
            "slow",
            "-pix_fmt",
            "yuv420p",
            "-r",
            "60",
            "-g",
            "30",
            "-bf",
            "2",
            "-b:v",
            "35M",
            "-maxrate",
            "40M",
            "-bufsize",
            "80M",
            "-colorspace",
            "bt709",
            "-color_primaries",
            "bt709",
            "-color_trc",
            "bt709",
            "-c:a",
            "aac",
            "-b:a",
            "384k",
            "-ar",
            "48000",
            "-ac",
            "2",
            "-movflags",
            "+faststart",
            "-shortest",
        ])
        .arg(out_mp4)
        .status()
        .context("spawn ffmpeg for video build")?;
    if !status.success() {
        anyhow::bail!("ffmpeg video failed ({status})");
    }
    println!("video: {}", out_mp4.display());
    Ok(())
}

/// Generate a folk-sequence-compatible schedule.json for `count` uploads on
/// Sun/Tue/Fri at 14:00 UTC. Shells to python3 for date arithmetic to avoid
/// a chrono dependency.
fn dispatch_schedule(start: Option<&str>, count: u32, out: &std::path::Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs_err::create_dir_all(parent)?;
        }
    }
    let start_arg = start.unwrap_or("");
    let script = r#"
import datetime as dt, json, sys
start_arg, count, out_path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
if start_arg:
    d = dt.date.fromisoformat(start_arg)
else:
    d = dt.date.today()
# Sun=6, Mon=0, Tue=1, Wed=2, Thu=3, Fri=4, Sat=5. Want {6,1,4}.
slots = {6, 1, 4}  # Sun, Tue, Fri
while d.weekday() not in slots:
    d += dt.timedelta(days=1)
entries = []
i = 0
while len(entries) < count:
    if d.weekday() in slots:
        entries.append({
            "episode": f"{len(entries)+1:03d}",
            "publish_at": f"{d.isoformat()}T14:00:00+00:00",
        })
    d += dt.timedelta(days=1)
    i += 1
with open(out_path, "w") as f:
    json.dump(entries, f, indent=2)
    f.write("\n")
print(f"wrote {len(entries)} entries spanning {entries[0]['publish_at'][:10]} -> {entries[-1]['publish_at'][:10]}")
"#;
    let status = std::process::Command::new("python3")
        .arg("-c")
        .arg(script)
        .arg(start_arg)
        .arg(count.to_string())
        .arg(out)
        .status()
        .context("spawn python3 for schedule generation")?;
    if !status.success() {
        anyhow::bail!("python3 schedule script failed ({status})");
    }
    Ok(())
}

/// Two-pass ffmpeg loudnorm master. Pass 1 measures, pass 2 applies linear
/// gain correction. ffmpeg is used here as a mastering-chain tool — distinct
/// from the five core synthesis/DSP tools (MBROLA, Praat, openSMILE,
/// Silero-VAD, csdr) that build the audio — per the user's explicit call in
/// the release plan. See docs/RELEASE_PLAN.md.
fn dispatch_master(
    in_wav: &std::path::Path,
    out_wav: &std::path::Path,
    lufs: f32,
    tp: f32,
    lra: f32,
) -> Result<()> {
    if !in_wav.exists() {
        anyhow::bail!("no such file: {}", in_wav.display());
    }
    let filter_pass1 = format!("loudnorm=I={lufs}:TP={tp}:LRA={lra}:print_format=json");
    let out = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-nostats", "-i"])
        .arg(in_wav)
        .args(["-af", &filter_pass1, "-f", "null", "-"])
        .output()
        .context("spawn ffmpeg pass 1")?;
    if !out.status.success() {
        anyhow::bail!(
            "ffmpeg pass 1 failed ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    // Parse JSON block from stderr (loudnorm prints to stderr)
    let stderr = String::from_utf8_lossy(&out.stderr);
    let start = stderr
        .rfind('{')
        .context("no JSON block in ffmpeg pass 1 output")?;
    let end = stderr[start..]
        .rfind('}')
        .context("no JSON close in ffmpeg pass 1 output")?
        + start
        + 1;
    let json_text = &stderr[start..end];
    let v: serde_json::Value = serde_json::from_str(json_text).context("parse loudnorm json")?;
    let measured_i = v["input_i"].as_str().unwrap_or("-24.0");
    let measured_lra = v["input_lra"].as_str().unwrap_or("11.0");
    let measured_tp = v["input_tp"].as_str().unwrap_or("-2.0");
    let measured_thresh = v["input_thresh"].as_str().unwrap_or("-34.0");
    let target_offset = v["target_offset"].as_str().unwrap_or("0.0");

    let filter_pass2 = format!(
        "loudnorm=I={lufs}:TP={tp}:LRA={lra}:measured_I={measured_i}:measured_LRA={measured_lra}:measured_TP={measured_tp}:measured_thresh={measured_thresh}:offset={target_offset}:linear=true:print_format=summary"
    );
    let status = std::process::Command::new("ffmpeg")
        .args(["-hide_banner", "-y", "-i"])
        .arg(in_wav)
        .args(["-af", &filter_pass2, "-c:a", "pcm_s24le"])
        .arg(out_wav)
        .status()
        .context("spawn ffmpeg pass 2")?;
    if !status.success() {
        anyhow::bail!("ffmpeg pass 2 failed ({status})");
    }
    println!(
        "mastered: {} ({measured_i} → {lufs} LUFS integrated)",
        out_wav.display()
    );
    Ok(())
}

/// Walk `albums/` to collect rendered track wavs in deterministic order.
/// Returns Vec<(album_slug, track_slug, wav_path)>.
fn collect_tracks(albums_filter: &[String]) -> Result<Vec<(String, String, std::path::PathBuf)>> {
    let root = std::path::Path::new("albums");
    let mut album_dirs: Vec<std::path::PathBuf> = fs_err::read_dir(root)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    album_dirs.sort();
    let mut out = Vec::new();
    for album_dir in album_dirs {
        let album_slug = album_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if !albums_filter.is_empty() && !albums_filter.contains(&album_slug) {
            continue;
        }
        let mut track_entries: Vec<std::path::PathBuf> = fs_err::read_dir(&album_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        track_entries.sort();
        for td in track_entries {
            let track_slug = td
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let wav = td.join(format!("{track_slug}.wav"));
            if wav.exists() {
                out.push((album_slug.clone(), track_slug, wav));
            }
        }
    }
    Ok(out)
}

fn wav_duration_s(path: &std::path::Path) -> Result<f32> {
    let r = hound::WavReader::open(path)?;
    Ok(r.duration() as f32 / r.spec().sample_rate as f32)
}

fn dispatch_compile(
    albums: &[String],
    target_minutes: f32,
    xfade_ms: u32,
    out_dir: &std::path::Path,
) -> Result<()> {
    use nadir_render::wav_to_f32;
    fs_err::create_dir_all(out_dir)?;
    let target_s = target_minutes * 60.0;
    let tracks = collect_tracks(albums)?;
    if tracks.is_empty() {
        anyhow::bail!("no rendered tracks found");
    }
    let mut manifest_lines: Vec<String> = Vec::new();

    // Pre-measure durations so greedy-pack can decide include/exclude per track.
    let mut durations: Vec<f32> = Vec::with_capacity(tracks.len());
    for (_, _, p) in &tracks {
        durations.push(wav_duration_s(p)?);
    }

    let mut i = 0usize;
    let mut compile_idx = 1usize;
    while i < tracks.len() {
        let mut bundle: Vec<usize> = vec![i];
        let mut total = durations[i];
        i += 1;
        while i < tracks.len() {
            let with = total + durations[i];
            let without = total;
            // Decide: pick whichever leaves total closer to target_s.
            let err_with = (with - target_s).abs();
            let err_without = (without - target_s).abs();
            if err_with <= err_without {
                bundle.push(i);
                total = with;
                i += 1;
                // If adding next would only make it worse, stop
                if total >= target_s {
                    // Evaluate one more step
                    if i < tracks.len() {
                        let next_err = (total + durations[i] - target_s).abs();
                        if next_err > err_with {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        // Write compilation
        let out_path = out_dir.join(format!("{compile_idx:03}.wav"));
        let wavs: Vec<&std::path::Path> = bundle.iter().map(|&k| tracks[k].2.as_path()).collect();
        let (total_written, spec) = chain_wavs_with_xfade(&wavs, xfade_ms, &out_path)?;
        println!(
            "{} — {:.2} min ({} tracks, {} ch, {} Hz)",
            out_path.display(),
            total_written / 60.0,
            bundle.len(),
            spec.channels,
            spec.sample_rate
        );
        // Manifest lines
        manifest_lines.push(format!(
            "[compilation.{compile_idx:03}]\nfile = \"{}\"\nduration_s = {:.3}\ntracks = [",
            out_path.display(),
            total_written
        ));
        for &k in &bundle {
            let (a, t, p) = &tracks[k];
            manifest_lines.push(format!(
                "  \"{a}/{t}/{}\",",
                p.file_name().unwrap().to_string_lossy()
            ));
        }
        manifest_lines.push("]\n".into());
        compile_idx += 1;
    }
    fs_err::write(out_dir.join("manifest.toml"), manifest_lines.join("\n"))?;
    println!(
        "wrote {} compilations to {}",
        compile_idx - 1,
        out_dir.display()
    );
    let _ = wav_to_f32; // reserved for future master step wiring
    Ok(())
}

/// Concatenate WAVs with a linear equal-power crossfade. All inputs must share
/// channel count and sample rate. Writes a single WAV out.
fn chain_wavs_with_xfade(
    wavs: &[&std::path::Path],
    xfade_ms: u32,
    out_path: &std::path::Path,
) -> Result<(f32, hound::WavSpec)> {
    if wavs.is_empty() {
        anyhow::bail!("no wavs to chain");
    }
    let first = hound::WavReader::open(wavs[0])?;
    let spec = first.spec();
    drop(first);
    let sr = spec.sample_rate as f32;
    let ch = spec.channels as usize;
    let xfade_n = (xfade_ms as f32 / 1000.0 * sr) as usize;

    let mut out: Vec<f32> = Vec::new();
    for (i, p) in wavs.iter().enumerate() {
        let (samples, this_sr) = read_wav_interleaved_f32(p)?;
        if this_sr != spec.sample_rate {
            anyhow::bail!("sample rate mismatch: {} vs {}", this_sr, spec.sample_rate);
        }
        if i == 0 {
            out.extend_from_slice(&samples);
            continue;
        }
        // Crossfade: fade out last xfade_n frames of `out` and fade in first
        // xfade_n frames of `samples`, then append the rest.
        let cur_frames = out.len() / ch;
        let inc_frames = samples.len() / ch;
        let x = xfade_n.min(cur_frames).min(inc_frames);
        for f in 0..x {
            let t = f as f32 / x as f32;
            let g_out = (1.0 - t).sqrt();
            let g_in = t.sqrt();
            for c in 0..ch {
                let out_idx = (cur_frames - x + f) * ch + c;
                let in_idx = f * ch + c;
                out[out_idx] = out[out_idx] * g_out + samples[in_idx] * g_in;
            }
        }
        // Append frames after the crossfade portion
        let tail_start_frame = x;
        let tail_start_sample = tail_start_frame * ch;
        out.extend_from_slice(&samples[tail_start_sample..]);
    }

    // Write interleaved
    let mut w = hound::WavWriter::create(out_path, spec)?;
    match spec.sample_format {
        hound::SampleFormat::Int => {
            for s in &out {
                let v = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
                w.write_sample(v)?;
            }
        }
        hound::SampleFormat::Float => {
            for s in &out {
                w.write_sample(*s)?;
            }
        }
    }
    w.finalize()?;
    let total_s = (out.len() / ch) as f32 / sr;
    Ok((total_s, spec))
}

fn read_wav_interleaved_f32(path: &std::path::Path) -> Result<(Vec<f32>, u32)> {
    let r = hound::WavReader::open(path)?;
    let spec = r.spec();
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            let bits = spec.bits_per_sample as i32;
            let max = (1i64 << (bits - 1)) as f32;
            hound::WavReader::open(path)?
                .into_samples::<i32>()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|v| v as f32 / max)
                .collect()
        }
        hound::SampleFormat::Float => hound::WavReader::open(path)?
            .into_samples::<f32>()
            .collect::<Result<Vec<_>, _>>()?,
    };
    Ok((samples, spec.sample_rate))
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
        AlbumSub::Render {
            slug,
            only_with_lyrics,
            keep_going,
        } => {
            let album_dir = std::path::Path::new("albums").join(&slug);
            let mut tracks: Vec<(u8, std::path::PathBuf)> = Vec::new();
            for entry in fs_err::read_dir(&album_dir)? {
                let entry = entry?;
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = entry.file_name();
                let name = name.to_string_lossy();
                // Expect NN_slug
                if let Some((num, _)) = name.split_once('_') {
                    if let Ok(n) = num.parse::<u8>() {
                        if only_with_lyrics {
                            let l = path.join("lyric.txt");
                            let txt = fs_err::read_to_string(&l).unwrap_or_default();
                            if txt.trim().is_empty() {
                                continue;
                            }
                        }
                        tracks.push((n, path));
                    }
                }
            }
            tracks.sort_by_key(|(n, _)| *n);
            let total = tracks.len();
            let mut rendered = 0usize;
            for (n, _path) in &tracks {
                println!("▸ [{n:02}/{total}] rendering track {n}");
                let exe = std::env::current_exe()?;
                let status = std::process::Command::new(&exe)
                    .args([
                        "song",
                        "render",
                        "--album",
                        &slug,
                        "--track",
                        &n.to_string(),
                    ])
                    .status()
                    .context("spawn self for song render")?;
                if status.success() {
                    rendered += 1;
                } else if !keep_going {
                    anyhow::bail!("track {n} failed; pass --keep-going to continue");
                } else {
                    tracing::warn!(track = n, "failed but keeping going");
                }
            }
            println!("rendered {rendered}/{total} tracks");
            Ok(())
        }
        AlbumSub::Play { slug } => {
            let album_dir = std::path::Path::new("albums").join(&slug);
            let mut tracks: Vec<(u8, std::path::PathBuf)> = Vec::new();
            for entry in fs_err::read_dir(&album_dir)? {
                let entry = entry?;
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name_owned = entry.file_name();
                let name = name_owned.to_string_lossy();
                let Some((num, _)) = name.split_once('_') else {
                    continue;
                };
                let Ok(n) = num.parse::<u8>() else { continue };
                // Prefer new <NN_slug>.wav naming; fall back to legacy render.wav.
                let new_render = path.join(format!("{name}.wav"));
                let legacy = path.join("render.wav");
                let render = if new_render.exists() {
                    new_render
                } else if legacy.exists() {
                    legacy
                } else {
                    continue;
                };
                tracks.push((n, render));
            }
            tracks.sort_by_key(|(n, _)| *n);
            let total = tracks.len();
            for (i, (n, path)) in tracks.iter().enumerate() {
                println!("▸ [{}/{total}] track {n}: {}", i + 1, path.display());
                dispatch_play(path)?;
            }
            Ok(())
        }
        AlbumSub::Audit { slug } => {
            let album_dir = std::path::Path::new("albums").join(&slug);
            let mut rows: Vec<(u8, String, Option<serde_json::Value>)> = Vec::new();
            for entry in fs_err::read_dir(&album_dir)? {
                let entry = entry?;
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = entry.file_name();
                let name = name.to_string_lossy().into_owned();
                let Some((num_str, slug_tail)) = name.split_once('_') else {
                    continue;
                };
                let Ok(n) = num_str.parse::<u8>() else {
                    continue;
                };
                let audit_path = if path.join("work/audit.json").exists() {
                    path.join("work/audit.json")
                } else {
                    path.join("stems/audit.json")
                };
                let audit = fs_err::read_to_string(&audit_path)
                    .ok()
                    .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok());
                rows.push((n, slug_tail.to_string(), audit));
            }
            rows.sort_by_key(|(n, _, _)| *n);
            println!(
                "{:<3}  {:<30}  {:>9}  {:>9}  {:>7}",
                "#", "slug", "rms_cents", "ceiling", "passed"
            );
            for (n, slug_tail, audit) in &rows {
                match audit {
                    Some(a) => {
                        let rms = a
                            .get("rms_cents")
                            .and_then(|x| x.as_f64())
                            .unwrap_or(f64::NAN);
                        let ceil = a
                            .get("ceiling_cents")
                            .and_then(|x| x.as_f64())
                            .unwrap_or(f64::NAN);
                        let passed = a.get("passed").and_then(|x| x.as_bool()).unwrap_or(false);
                        println!(
                            "{:02}   {:<30}  {:>9.1}  {:>9.1}  {:>7}",
                            n,
                            slug_tail,
                            rms,
                            ceil,
                            if passed { "yes" } else { "NO" }
                        );
                    }
                    None => {
                        println!(
                            "{:02}   {:<30}  {:>9}  {:>9}  {:>7}",
                            n, slug_tail, "—", "—", "—"
                        );
                    }
                }
            }
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
        SongSub::Render {
            album,
            track,
            out,
            voice,
            bpm,
            key,
            scale,
            seed,
            bed_preset,
            strict,
        } => {
            use nadir_compose::{plan_melody_phrased_in_range, render_vox_pho_phrased};
            use nadir_core::{Key, Scale, ScaleKind};
            use nadir_praat::{extract_f0_script, psola_retarget_script, run_inline, PraatConfig};
            use nadir_vox::{synth_to_wav, MbrolaConfig};
            use serde::Deserialize;
            use std::process::Command;
            use std::str::FromStr;

            // Kept for compatibility but superseded by FullManifest.
            #[allow(dead_code)]
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
                #[serde(default = "default_bars")]
                bars: u32,
                #[serde(default = "default_meter")]
                meter: [u32; 2],
                #[serde(default)]
                title: Option<String>,
                /// Repeat the composed phrase stream N times end-to-end.
                /// Stretches short lyrics toward the 3-min target from the
                /// 100-video release plan.
                #[serde(default = "default_repeat")]
                section_repeat: u32,
            }
            #[derive(Deserialize, Default, Clone)]
            struct SecondaryVoice {
                voice: String,
                #[serde(default)]
                octave: i32,
                #[serde(default = "default_secondary_gain")]
                gain: f32,
                #[serde(default)]
                pan: f32,
            }
            #[derive(Deserialize, Default)]
            struct DspFields {
                #[serde(default)]
                bed_preset: Option<String>,
                /// Stack multiple bed presets (layered noise + tonal drone etc).
                /// If set, takes precedence over bed_preset.
                #[serde(default)]
                bed_presets: Option<Vec<String>>,
                #[serde(default = "default_bed_gain")]
                bed_gain: f32,
                #[serde(default = "default_vocal_gain")]
                vocal_gain: f32,
                #[serde(default = "default_pulse")]
                pulses: bool,
                #[serde(default = "default_pulse_gain")]
                pulse_gain: f32,
                #[serde(default = "default_pulse_ms")]
                pulse_ms: u32,
                #[serde(default = "default_pulse_kind")]
                pulse_kind: String,
                #[serde(default)]
                vocal_pan: f32,
                #[serde(default = "default_bed_pan")]
                bed_pan: f32,
                #[serde(default = "default_pulse_pan")]
                pulse_pan: f32,
                #[serde(default = "default_pulse_pingpong")]
                pulse_pingpong: bool,
                #[serde(default = "default_pulse_pingpong_width")]
                pulse_pingpong_width: f32,
                #[serde(default = "default_on")]
                kick: bool,
                #[serde(default = "default_on")]
                hat: bool,
                #[serde(default = "default_on")]
                bass_arp: bool,
                #[serde(default = "default_on")]
                oohs: bool,
                #[serde(default = "default_oohs_vowel")]
                oohs_vowel: String,
                #[serde(default = "default_echo_on")]
                echo: bool,
                #[serde(default = "default_echo_taps")]
                echo_taps: Vec<(u32, f32)>,
                #[serde(default)]
                secondary_voices: Vec<SecondaryVoice>,
            }
            fn default_bpm() -> f32 {
                96.0
            }
            fn default_bars() -> u32 {
                16
            }
            fn default_meter() -> [u32; 2] {
                [4, 4]
            }
            fn default_repeat() -> u32 {
                1
            }
            fn default_voice() -> String {
                "us1".into()
            }
            fn default_seed() -> u64 {
                42
            }
            fn default_bed_gain() -> f32 {
                0.35
            }
            fn default_vocal_gain() -> f32 {
                0.9
            }
            fn default_pulse() -> bool {
                true
            }
            fn default_pulse_gain() -> f32 {
                0.9
            }
            fn default_pulse_ms() -> u32 {
                80
            }
            fn default_secondary_gain() -> f32 {
                0.4
            }
            fn default_pulse_kind() -> String {
                "tonic".into()
            }
            fn default_bed_pan() -> f32 {
                0.0
            }
            fn default_pulse_pan() -> f32 {
                0.0
            }
            fn default_pulse_pingpong() -> bool {
                true
            }
            fn default_pulse_pingpong_width() -> f32 {
                0.55
            }
            fn default_echo_on() -> bool {
                true
            }
            fn default_on() -> bool {
                true
            }
            fn default_oohs_vowel() -> String {
                // "u" is the richest low-formant MBROLA vowel across us1/us2/us3.
                "u".into()
            }
            fn default_echo_taps() -> Vec<(u32, f32)> {
                // (delay_ms, gain). Quick comb of small mid-time echoes ≈ hall sense.
                vec![(187, 0.22), (311, 0.14), (523, 0.09)]
            }
            #[derive(Deserialize, Default)]
            struct TargetsFields {
                #[serde(default)]
                pitch_error_ceiling_cents: Option<f32>,
                #[serde(default)]
                vox_loudness_lufs: Option<f32>,
                #[serde(default)]
                bed_loudness_lufs: Option<f32>,
                #[serde(default)]
                pulse_loudness_lufs: Option<f32>,
                #[serde(default)]
                tessitura_hz: Option<[f32; 2]>,
            }
            #[derive(Deserialize)]
            struct FullManifest {
                track: TrackFields,
                #[serde(default)]
                dsp: Option<DspFields>,
                #[serde(default)]
                targets: Option<TargetsFields>,
            }

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
            let mut m: FullManifest = toml::from_str(&manifest_text)?;
            // Apply CLI overrides onto manifest fields
            if let Some(v) = voice.as_ref() {
                m.track.mbrola_voice = v.clone();
            }
            if let Some(b) = bpm {
                m.track.bpm = b;
            }
            if let Some(k) = key.as_ref() {
                m.track.key = k.clone();
            }
            if let Some(s) = scale.as_ref() {
                m.track.scale = s.clone();
            }
            if let Some(sd) = seed {
                m.track.seed = sd;
            }
            let mut dsp_cfg = m.dsp.unwrap_or(DspFields {
                bed_preset: None,
                bed_presets: None,
                bed_gain: default_bed_gain(),
                vocal_gain: default_vocal_gain(),
                pulses: default_pulse(),
                pulse_gain: default_pulse_gain(),
                pulse_ms: default_pulse_ms(),
                pulse_kind: default_pulse_kind(),
                vocal_pan: 0.0,
                bed_pan: default_bed_pan(),
                pulse_pan: default_pulse_pan(),
                pulse_pingpong: default_pulse_pingpong(),
                pulse_pingpong_width: default_pulse_pingpong_width(),
                kick: default_on(),
                hat: default_on(),
                bass_arp: default_on(),
                oohs: default_on(),
                oohs_vowel: default_oohs_vowel(),
                echo: default_echo_on(),
                echo_taps: default_echo_taps(),
                secondary_voices: Vec::new(),
            });
            if let Some(bp) = bed_preset.as_ref() {
                dsp_cfg.bed_preset = Some(bp.clone());
            }
            let raw_lyric = fs_err::read_to_string(track_dir.join("lyric.txt")).unwrap_or_default();
            let phrases: Vec<Vec<String>> = raw_lyric
                .lines()
                .filter_map(|l| {
                    let words: Vec<String> = l.split_whitespace().map(str::to_string).collect();
                    if words.is_empty() {
                        None
                    } else {
                        Some(words)
                    }
                })
                .collect();
            let lyric = phrases
                .iter()
                .flatten()
                .cloned()
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
                .args([
                    "run",
                    "--project",
                    "python/nadir-lyric-g2p",
                    "nadir-g2p",
                    "--stress",
                    "--voice",
                    &m.track.mbrola_voice,
                    "--text",
                    &lyric,
                ])
                .output()
                .context("g2p spawn")?;
            if !g2p_out.status.success() {
                anyhow::bail!("g2p: {}", String::from_utf8_lossy(&g2p_out.stderr));
            }
            let word_data: Vec<serde_json::Value> = serde_json::from_slice(&g2p_out.stdout)?;
            let phonemes: Vec<Vec<String>> = word_data
                .iter()
                .map(|v| {
                    v["phonemes"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|x| x.as_str().map(str::to_string))
                        .collect()
                })
                .collect();
            let stresses: Vec<f32> = word_data
                .iter()
                .map(|v| v["stress"].as_f64().unwrap_or(1.0) as f32)
                .collect();
            let syllables: Vec<String> = lyric.split_whitespace().map(str::to_string).collect();
            let phrase_lens: Vec<usize> = phrases.iter().map(|p| p.len()).collect();

            let tessitura: Option<(f32, f32)> = m
                .targets
                .as_ref()
                .and_then(|t| t.tessitura_hz)
                .map(|[lo, hi]| (lo, hi));
            let base_notes = plan_melody_phrased_in_range(
                &sc,
                &syllables,
                &phrase_lens,
                m.track.seed,
                220.0,
                m.track.bpm,
                &stresses,
                tessitura,
            );
            // Section repeat — cycle the composed phrases N times to stretch
            // short lyrics toward the 3-min target. Notes, phonemes, stresses,
            // syllables, and phrase_lens all get repeated N times in lockstep
            // so downstream steps (render, syllable dynamics, oohs) stay aligned.
            let repeat = m.track.section_repeat.max(1);
            let base_phonemes = phonemes.clone();
            let base_stresses = stresses.clone();
            let base_syllables = syllables.clone();
            let base_phrase_lens = phrase_lens.clone();
            let mut notes = base_notes.clone();
            let mut phonemes = base_phonemes.clone();
            let mut stresses = base_stresses.clone();
            let mut syllables = base_syllables.clone();
            let mut phrase_lens = base_phrase_lens.clone();
            for _ in 1..repeat {
                notes.extend_from_slice(&base_notes);
                phonemes.extend(base_phonemes.iter().cloned());
                stresses.extend(base_stresses.iter().copied());
                syllables.extend(base_syllables.iter().cloned());
                phrase_lens.extend(base_phrase_lens.iter().copied());
            }
            let stream = render_vox_pho_phrased(&notes, &phonemes, &phrase_lens, 30, 400);

            let vox_cfg = MbrolaConfig {
                voice: m.track.mbrola_voice.clone(),
                ..Default::default()
            };

            let stems_dir = track_dir.join("stems");
            let work_dir = track_dir.join("work");
            fs_err::create_dir_all(&stems_dir)?;
            fs_err::create_dir_all(&work_dir)?;
            // Intermediates live in work/; production stems live in stems/.
            let raw_vox_path = work_dir.join("raw_vox.wav");
            let tuned_vox_path = stems_dir.join("vocal.wav");
            let f0_realized_path = work_dir.join("f0_realized.csv");
            let f0_target_path = work_dir.join("f0_target.csv");

            synth_to_wav(&vox_cfg, &stream, &raw_vox_path)?;

            let praat_cfg = PraatConfig::default();
            run_inline(
                &praat_cfg,
                &extract_f0_script(&raw_vox_path, &f0_realized_path),
                &[],
            )?;
            let f0_text = fs_err::read_to_string(&f0_realized_path)?;
            let realized: Vec<(f32, f32)> = f0_text
                .lines()
                .skip(1)
                .filter_map(|l| {
                    let mut it = l.split(',');
                    let t: f32 = it.next()?.parse().ok()?;
                    let h: f32 = it.next()?.parse().ok()?;
                    Some((t, h))
                })
                .collect();

            let dest = if out.to_str() == Some("out.wav") {
                let dir_name = track_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("render");
                track_dir.join(format!("{dir_name}.wav"))
            } else {
                out.clone()
            };

            if realized.is_empty() {
                fs_err::copy(&raw_vox_path, &tuned_vox_path)?;
            } else {
                {
                    use std::io::Write;
                    let mut f = std::fs::File::create(&f0_target_path)?;
                    writeln!(f, "time_s,hz")?;
                    // Dead-zone: if realized is already within ±15 cents of the
                    // nearest scale degree, use realized (don't ask PSOLA to
                    // move it). Avoids introducing tracker-quantization jitter
                    // on frames that were already in tune.
                    let dead_zone_cents = 15.0_f32;
                    for (t, hz) in &realized {
                        let snapped = sc.snap(*hz);
                        let cents = 1200.0 * (hz / snapped).ln() / std::f32::consts::LN_2;
                        let target = if cents.abs() <= dead_zone_cents {
                            *hz
                        } else {
                            snapped
                        };
                        writeln!(f, "{t},{target}")?;
                    }
                }
                let script = psola_retarget_script(&raw_vox_path, &f0_target_path, &tuned_vox_path);
                run_inline(&praat_cfg, &script, &[])?;
            }

            // ── secondary voices (duet stack) ──
            let mut secondary_stems: Vec<(Vec<f32>, f32, f32)> = Vec::new(); // (samples, gain, pan)
            for sv in &dsp_cfg.secondary_voices {
                // Re-G2P for this voice (lexicon maps to voice-appropriate phonemes)
                let sv_g2p = Command::new("uv")
                    .args([
                        "run",
                        "--project",
                        "python/nadir-lyric-g2p",
                        "nadir-g2p",
                        "--stress",
                        "--voice",
                        &sv.voice,
                        "--text",
                        &lyric,
                    ])
                    .output()
                    .context("g2p spawn (secondary)")?;
                if !sv_g2p.status.success() {
                    tracing::warn!(voice=%sv.voice, err=%String::from_utf8_lossy(&sv_g2p.stderr), "secondary voice g2p failed, skipping");
                    continue;
                }
                let sv_words: Vec<serde_json::Value> = serde_json::from_slice(&sv_g2p.stdout)?;
                let sv_phonemes: Vec<Vec<String>> = sv_words
                    .iter()
                    .map(|v| {
                        v["phonemes"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|x| x.as_str().map(str::to_string))
                            .collect()
                    })
                    .collect();

                // Re-plan notes at octave-shifted center
                let shifted_center = 220.0 * 2f32.powi(sv.octave);
                let sv_notes = plan_melody_phrased_in_range(
                    &sc,
                    &syllables,
                    &phrase_lens,
                    m.track.seed.wrapping_add(0xB17AA11A),
                    shifted_center,
                    m.track.bpm,
                    &stresses,
                    tessitura,
                );
                let sv_stream =
                    render_vox_pho_phrased(&sv_notes, &sv_phonemes, &phrase_lens, 30, 400);
                let sv_cfg = MbrolaConfig {
                    voice: sv.voice.clone(),
                    ..Default::default()
                };
                let sv_raw = work_dir.join(format!("raw_vox_{}.wav", sv.voice));
                let sv_tuned = stems_dir.join(format!("vocal_{}.wav", sv.voice));
                if let Err(e) = synth_to_wav(&sv_cfg, &sv_stream, &sv_raw) {
                    tracing::warn!(voice=%sv.voice, err=%e, "secondary synth failed, skipping");
                    continue;
                }
                let sv_f0_csv = work_dir.join(format!("f0_realized_{}.csv", sv.voice));
                run_inline(&praat_cfg, &extract_f0_script(&sv_raw, &sv_f0_csv), &[])?;
                let sv_f0_text = fs_err::read_to_string(&sv_f0_csv)?;
                let sv_realized: Vec<(f32, f32)> = sv_f0_text
                    .lines()
                    .skip(1)
                    .filter_map(|l| {
                        let mut it = l.split(',');
                        let t: f32 = it.next()?.parse().ok()?;
                        let h: f32 = it.next()?.parse().ok()?;
                        Some((t, h))
                    })
                    .collect();
                if sv_realized.is_empty() {
                    fs_err::copy(&sv_raw, &sv_tuned)?;
                } else {
                    let sv_target_csv = work_dir.join(format!("f0_target_{}.csv", sv.voice));
                    {
                        use std::io::Write;
                        let mut f = std::fs::File::create(&sv_target_csv)?;
                        writeln!(f, "time_s,hz")?;
                        for (t, hz) in &sv_realized {
                            let snapped = sc.snap(*hz);
                            writeln!(f, "{t},{snapped}")?;
                        }
                    }
                    let sv_script = psola_retarget_script(&sv_raw, &sv_target_csv, &sv_tuned);
                    run_inline(&praat_cfg, &sv_script, &[])?;
                }
                let samples = nadir_render::upsample_16_to_48_via_csdr(&sv_tuned)?;
                secondary_stems.push((samples, sv.gain, sv.pan));
            }

            // ── oohs (vowel-only held pad voice) ──
            let oohs_stem_48k: Option<Vec<f32>> = if dsp_cfg.oohs {
                // Substitute phonemes with just [oohs_vowel] per syllable; same
                // plan_melody output drives pitch. MBROLA renders held vowels.
                let vowel = dsp_cfg.oohs_vowel.clone();
                let oohs_phonemes: Vec<Vec<String>> =
                    (0..notes.len()).map(|_| vec![vowel.clone()]).collect();
                let oohs_stream = nadir_compose::render_vox_pho_phrased(
                    &notes,
                    &oohs_phonemes,
                    &phrase_lens,
                    30,
                    400,
                );
                let oohs_raw = work_dir.join("raw_oohs.wav");
                let oohs_tuned = stems_dir.join("oohs.wav");
                let oohs_cfg = MbrolaConfig {
                    voice: m.track.mbrola_voice.clone(),
                    ..Default::default()
                };
                match synth_to_wav(&oohs_cfg, &oohs_stream, &oohs_raw) {
                    Ok(_) => {
                        // Same PSOLA retarget as primary: extract F0, snap, retarget.
                        let oohs_f0_csv = work_dir.join("f0_realized_oohs.csv");
                        run_inline(&praat_cfg, &extract_f0_script(&oohs_raw, &oohs_f0_csv), &[])?;
                        let oohs_f0_text = fs_err::read_to_string(&oohs_f0_csv)?;
                        let oohs_realized: Vec<(f32, f32)> = oohs_f0_text
                            .lines()
                            .skip(1)
                            .filter_map(|l| {
                                let mut it = l.split(',');
                                let t: f32 = it.next()?.parse().ok()?;
                                let h: f32 = it.next()?.parse().ok()?;
                                Some((t, h))
                            })
                            .collect();
                        if oohs_realized.is_empty() {
                            fs_err::copy(&oohs_raw, &oohs_tuned)?;
                        } else {
                            let oohs_target_csv = work_dir.join("f0_target_oohs.csv");
                            {
                                use std::io::Write;
                                let mut f = std::fs::File::create(&oohs_target_csv)?;
                                writeln!(f, "time_s,hz")?;
                                for (t, hz) in &oohs_realized {
                                    let snapped = sc.snap(*hz);
                                    writeln!(f, "{t},{snapped}")?;
                                }
                            }
                            let script =
                                psola_retarget_script(&oohs_raw, &oohs_target_csv, &oohs_tuned);
                            run_inline(&praat_cfg, &script, &[])?;
                        }
                        Some(nadir_render::upsample_16_to_48_via_csdr(&oohs_tuned)?)
                    }
                    Err(e) => {
                        tracing::warn!(err=%e, "oohs synth failed, skipping");
                        None
                    }
                }
            } else {
                None
            };

            // ── bed + pulses + mix ──
            let vocal_info = hound::WavReader::open(&tuned_vox_path)?;
            let dur_s = vocal_info.duration() as f32 / vocal_info.spec().sample_rate as f32;
            drop(vocal_info);

            let bed_path = stems_dir.join("bed.wav");
            let pulses_path = stems_dir.join("pulses.wav");
            // Secondary voices → stems/vocal_<voice>.wav (set below)

            // Resolve which preset names to stack. bed_presets list wins if set,
            // else single bed_preset, else none.
            let preset_names: Vec<String> = match (&dsp_cfg.bed_presets, &dsp_cfg.bed_preset) {
                (Some(list), _) if !list.is_empty() => list.clone(),
                (_, Some(name)) => vec![name.clone()],
                _ => Vec::new(),
            };
            let mut bed: Option<Vec<f32>> = None;
            for name in &preset_names {
                let sub: Option<Vec<f32>> = match nadir_render::resolve_bed(name) {
                    Some(nadir_render::BedKind::ShapedNoise { low, high, tilt }) => {
                        Some(nadir_render::bed_shaped_noise(dur_s, low, high, tilt)?)
                    }
                    Some(nadir_render::BedKind::TonalTriad { octave, fade_s }) => {
                        let b = nadir_render::bed_tonal_triad(&sc, dur_s, octave, fade_s);
                        let low = 60.0 / nadir_render::MASTER_SR as f32;
                        let high = 2500.0 / nadir_render::MASTER_SR as f32;
                        Some(nadir_render::band_limit_via_csdr(&b, low, high, 0.02).unwrap_or(b))
                    }
                    None => {
                        tracing::warn!(bed_preset=%name, "unknown bed preset, skipping");
                        None
                    }
                };
                if let Some(s) = sub {
                    bed = match bed {
                        None => Some(s),
                        Some(mut acc) => {
                            let n = acc.len().max(s.len());
                            acc.resize(n, 0.0);
                            for (i, v) in s.iter().enumerate() {
                                acc[i] += v;
                            }
                            Some(acc)
                        }
                    };
                }
            }
            if let Some(ref mut b) = bed {
                // Slow breath tremolo at 0.22 Hz, 30% depth — gives the bed life.
                nadir_render::amp_tremolo(b, 0.22, 0.30);
                nadir_render::f32_to_wav_s16(b, nadir_render::MASTER_SR, &bed_path)?;
            }

            // Returns Vec<(samples, pan)> — one entry if pingpong off, two if on.
            let mut pulse_stems: Vec<(Vec<f32>, f32)> = if dsp_cfg.pulses {
                use nadir_vad::{detect_onsets, VadConfig};
                let vad_cfg = VadConfig::default();
                match detect_onsets(&vad_cfg, &tuned_vox_path, Some(m.track.bpm)) {
                    Ok(onsets) => {
                        // Combine VAD-driven onsets (vocal syllable triggers) with
                        // a quarter-note beat grid driven by BPM. Dense result:
                        // vocal accents on top of a steady pulse.
                        let mut times: Vec<f32> = onsets.iter().map(|o| o.time_s).collect();
                        let beat_s = 60.0 / m.track.bpm.max(1.0);
                        let mut t = beat_s * 0.5; // start on the backbeat
                        while t < dur_s - 0.1 {
                            times.push(t);
                            t += beat_s;
                        }
                        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        // Dedupe near-duplicates within 50 ms to avoid flams
                        times.dedup_by(|a, b| (*a - *b).abs() < 0.05);
                        let build = |ts: &[f32]| -> Vec<f32> {
                            let raw = match dsp_cfg.pulse_kind.as_str() {
                                "tonic" => {
                                    let tonic = sc.degrees_hz(-2).first().copied().unwrap_or(55.0);
                                    nadir_render::pulse_track_pitched(
                                        ts,
                                        dur_s,
                                        dsp_cfg.pulse_ms.max(60),
                                        tonic,
                                    )
                                }
                                _ => nadir_render::pulse_track(
                                    ts,
                                    dur_s,
                                    dsp_cfg.pulse_ms,
                                    m.track.seed,
                                ),
                            };
                            let (low, high) = match dsp_cfg.pulse_kind.as_str() {
                                "tonic" => (
                                    40.0 / nadir_render::MASTER_SR as f32,
                                    500.0 / nadir_render::MASTER_SR as f32,
                                ),
                                _ => (
                                    200.0 / nadir_render::MASTER_SR as f32,
                                    2000.0 / nadir_render::MASTER_SR as f32,
                                ),
                            };
                            nadir_render::band_limit_via_csdr(&raw, low, high, 0.01).unwrap_or(raw)
                        };
                        if dsp_cfg.pulse_pingpong && times.len() >= 2 {
                            let (even, odd) = nadir_render::split_onsets_even_odd(&times);
                            let left = build(&even);
                            let right = build(&odd);
                            // Persist a mixed-down stem for stems/pulses.wav
                            let n = left.len().max(right.len());
                            let mut mono = vec![0.0f32; n];
                            for (i, v) in left.iter().enumerate() {
                                mono[i] += 0.5 * v;
                            }
                            for (i, v) in right.iter().enumerate() {
                                mono[i] += 0.5 * v;
                            }
                            nadir_render::f32_to_wav_s16(
                                &mono,
                                nadir_render::MASTER_SR,
                                &pulses_path,
                            )?;
                            let w = dsp_cfg.pulse_pingpong_width;
                            vec![(left, -w), (right, w)]
                        } else {
                            let shaped = build(&times);
                            nadir_render::f32_to_wav_s16(
                                &shaped,
                                nadir_render::MASTER_SR,
                                &pulses_path,
                            )?;
                            vec![(shaped, dsp_cfg.pulse_pan)]
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error=%e, "vad onsets failed, skipping pulses");
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            };

            // ── rhythmic stems (kick / hat / bass_arp), BPM-driven, key-aware ──
            let kick_path = stems_dir.join("kick.wav");
            let hat_path = stems_dir.join("hat.wav");
            let bass_path = stems_dir.join("bass_arp.wav");
            let tonic_low = sc.degrees_hz(-2).first().copied().unwrap_or(55.0);
            let deg = sc.degrees_hz(-1);
            let kick_stem: Option<Vec<f32>> = if dsp_cfg.kick {
                let grid = nadir_render::beat_grid_times(m.track.bpm, dur_s, 1);
                let mut s = nadir_render::pulse_track_pitched(&grid, dur_s, 130, tonic_low);
                // Shape low-band
                let low = 40.0 / nadir_render::MASTER_SR as f32;
                let high = 200.0 / nadir_render::MASTER_SR as f32;
                s = nadir_render::band_limit_via_csdr(&s, low, high, 0.01).unwrap_or(s);
                nadir_render::f32_to_wav_s16(&s, nadir_render::MASTER_SR, &kick_path)?;
                Some(s)
            } else {
                None
            };
            let hat_stem: Option<Vec<f32>> = if dsp_cfg.hat {
                let grid = nadir_render::beat_grid_times(m.track.bpm, dur_s, 2);
                // Offset hat by half-16th to land on off-beats
                let grid_off: Vec<f32> = grid
                    .iter()
                    .map(|t| t + 60.0 / (m.track.bpm * 4.0))
                    .filter(|t| *t < dur_s)
                    .collect();
                let mut s = nadir_render::pulse_track(&grid_off, dur_s, 28, m.track.seed ^ 0xA7);
                let low = 3000.0 / nadir_render::MASTER_SR as f32;
                let high = 8000.0 / nadir_render::MASTER_SR as f32;
                s = nadir_render::band_limit_via_csdr(&s, low, high, 0.01).unwrap_or(s);
                nadir_render::f32_to_wav_s16(&s, nadir_render::MASTER_SR, &hat_path)?;
                Some(s)
            } else {
                None
            };
            let bass_stem: Option<Vec<f32>> = if dsp_cfg.bass_arp && !deg.is_empty() {
                // root, fifth, octave, fifth at -1 octave → arpeggio
                let root = deg[0];
                let fifth = deg.get(4).copied().unwrap_or(root * 1.5);
                let octave = root * 2.0;
                let cycle = vec![root, fifth, octave, fifth];
                let s = nadir_render::arp_track(&cycle, dur_s, m.track.bpm, 4, 100);
                let low = 60.0 / nadir_render::MASTER_SR as f32;
                let high = 500.0 / nadir_render::MASTER_SR as f32;
                let shaped = nadir_render::band_limit_via_csdr(&s, low, high, 0.01).unwrap_or(s);
                nadir_render::f32_to_wav_s16(&shaped, nadir_render::MASTER_SR, &bass_path)?;
                Some(shaped)
            } else {
                None
            };

            // ── per-stem loudness normalization (RMS proxy for LUFS) ──
            // Unified target: every production stem lands at -18 dBFS integrated
            // RMS. Secondaries sit a bit below so the primary vocal wins the
            // foreground. Override per stem with [targets].*_loudness_lufs.
            let stem_default: f32 = -18.0;
            let vox_target = m
                .targets
                .as_ref()
                .and_then(|t| t.vox_loudness_lufs)
                .unwrap_or(stem_default);
            let bed_target = m
                .targets
                .as_ref()
                .and_then(|t| t.bed_loudness_lufs)
                .unwrap_or(stem_default);
            let pulse_target = m
                .targets
                .as_ref()
                .and_then(|t| t.pulse_loudness_lufs)
                .unwrap_or(stem_default);
            // Stress → amplitude map (primary slightly louder, unstressed softer)
            let stress_gain = |stress: f32| -> f32 {
                if stress >= 1.15 {
                    1.20
                } else if stress < 0.9 {
                    0.72
                } else {
                    1.0
                }
            };
            let note_durs: Vec<u32> = notes.iter().map(|n| n.dur_ms).collect();

            // Vocal WAV on disk — apply per-syllable dynamics, then normalize
            {
                let (mut v, sr) = nadir_render::wav_to_f32(&tuned_vox_path)?;
                nadir_render::apply_syllable_dynamics(
                    &mut v,
                    sr,
                    &note_durs,
                    &phrase_lens,
                    &stresses,
                    400,
                    30,
                    15,
                    stress_gain,
                );
                nadir_render::normalize_to_dbfs(&mut v, vox_target, 18.0);
                nadir_render::f32_to_wav_s16(&v, sr, &tuned_vox_path)?;
            }
            if let Some(ref mut b) = bed {
                nadir_render::normalize_to_dbfs(b, bed_target, 18.0);
            }
            for (samples, _) in pulse_stems.iter_mut() {
                nadir_render::normalize_to_dbfs(samples, pulse_target, 18.0);
            }
            for (samples, _, _) in secondary_stems.iter_mut() {
                nadir_render::normalize_to_dbfs(samples, vox_target - 3.0, 18.0);
            }
            let mut kick_stem = kick_stem;
            if let Some(ref mut k) = kick_stem {
                nadir_render::normalize_to_dbfs(k, stem_default, 18.0);
                nadir_render::f32_to_wav_s16(k, nadir_render::MASTER_SR, &kick_path)?;
            }
            let mut hat_stem = hat_stem;
            if let Some(ref mut h) = hat_stem {
                nadir_render::normalize_to_dbfs(h, stem_default, 18.0);
                nadir_render::f32_to_wav_s16(h, nadir_render::MASTER_SR, &hat_path)?;
            }
            let mut bass_stem = bass_stem;
            if let Some(ref mut b) = bass_stem {
                nadir_render::normalize_to_dbfs(b, stem_default, 18.0);
                nadir_render::f32_to_wav_s16(b, nadir_render::MASTER_SR, &bass_path)?;
            }
            let mut oohs_stem_48k = oohs_stem_48k;
            if let Some(ref mut o) = oohs_stem_48k {
                nadir_render::normalize_to_dbfs(o, stem_default - 3.0, 18.0);
            }

            let mut stems: Vec<(&[f32], f32, f32)> = Vec::new(); // (samples, gain, pan)
            if let Some(ref b) = bed {
                stems.push((b.as_slice(), dsp_cfg.bed_gain, dsp_cfg.bed_pan));
            }
            for (samples, pan) in &pulse_stems {
                stems.push((samples.as_slice(), dsp_cfg.pulse_gain, *pan));
            }
            if let Some(ref k) = kick_stem {
                stems.push((k.as_slice(), 0.85, 0.0));
            }
            if let Some(ref h) = hat_stem {
                stems.push((h.as_slice(), 0.45, 0.25));
            }
            if let Some(ref b) = bass_stem {
                stems.push((b.as_slice(), 0.7, -0.15));
            }
            if let Some(ref o) = oohs_stem_48k {
                stems.push((o.as_slice(), 0.55, 0.1));
            }
            for (samples, gain, pan) in &secondary_stems {
                stems.push((samples.as_slice(), *gain, *pan));
            }
            let taps: Vec<(u32, f32)> = if dsp_cfg.echo {
                dsp_cfg.echo_taps.clone()
            } else {
                Vec::new()
            };
            if stems.is_empty() && dsp_cfg.vocal_pan == 0.0 && taps.is_empty() {
                fs_err::copy(&tuned_vox_path, &dest)?;
            } else {
                nadir_render::mix_stems_stereo_with_echo(
                    &tuned_vox_path,
                    &stems,
                    dsp_cfg.vocal_gain,
                    dsp_cfg.vocal_pan,
                    &taps,
                    &dest,
                )?;
            }

            // ── openSMILE audit ──
            let ceiling = m.targets.as_ref().and_then(|t| t.pitch_error_ceiling_cents);
            let audit_result =
                run_pitch_audit(&tuned_vox_path, &f0_target_path, &work_dir, ceiling);
            match audit_result {
                Ok(rms) => {
                    println!("audit: {rms:.1} cents rms");
                    if strict {
                        let c = ceiling.unwrap_or(30.0);
                        if rms > c {
                            anyhow::bail!("audit failed: rms_cents {rms:.1} > ceiling {c:.1}");
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(error=%e, "openSMILE audit failed (non-fatal)");
                }
            }

            // ── chord chart: abc + midi (tonic triad held each bar) ──
            let bars = m.track.bars.max(1);
            let meter = (m.track.meter[0] as u8, m.track.meter[1] as u8);
            let ivals = sc.kind.intervals();
            let root_midi: i32 = 60 + sc.key.semitone(); // middle octave
            let third_off = ivals.get(2).copied().unwrap_or(4) as i32;
            let fifth_off = ivals.get(4).copied().unwrap_or(7) as i32;
            let triad: Vec<u8> = [root_midi, root_midi + third_off, root_midi + fifth_off]
                .iter()
                .map(|n| (*n).clamp(0, 127) as u8)
                .collect();
            let chord_sym = {
                let letters = [
                    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
                ];
                let letter = letters[(sc.key.semitone().rem_euclid(12)) as usize];
                // Treat anything not "major" as minor for the rudimentary chord chart.
                let scale_str = format!("{:?}", sc.kind).to_lowercase();
                if scale_str.contains("major")
                    || matches!(
                        sc.kind,
                        nadir_core::ScaleKind::PentatonicMajor
                            | nadir_core::ScaleKind::Major
                            | nadir_core::ScaleKind::Lydian
                            | nadir_core::ScaleKind::Mixolydian
                    )
                {
                    letter.to_string()
                } else {
                    format!("{letter}m")
                }
            };
            let abc_path = work_dir.join("chords.abc");
            let midi_path = work_dir.join("chords.mid");
            let title = m.track.title.clone().unwrap_or_else(|| "untitled".into());
            let mut abc = String::new();
            abc.push_str(&format!(
                "X:1\nT:{title}\nM:{}/{}\nL:1/4\nQ:1/4={}\nK:{chord_sym}\n",
                meter.0, meter.1, m.track.bpm as u32
            ));
            for b in 0..bars {
                abc.push_str(&format!("|\"{chord_sym}\" z{} ", meter.0));
                if (b + 1) % 4 == 0 {
                    abc.push('\n');
                }
            }
            abc.push_str("|]\n");
            fs_err::write(&abc_path, abc)?;
            let chords: Vec<Vec<u8>> = (0..bars as usize).map(|_| triad.clone()).collect();
            nadir_render::write_chord_midi(&midi_path, m.track.bpm, meter, &chords)?;

            println!("{}", dest.display());
            println!("stems: {}", stems_dir.display());
            println!("work:  {}", work_dir.display());
            Ok(())
        }
        SongSub::Audit { album, track } => {
            let album_dir = std::path::Path::new("albums").join(&album);
            let mut track_dir: Option<std::path::PathBuf> = None;
            for entry in fs_err::read_dir(&album_dir)? {
                let entry = entry?;
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if let Some((num, _)) = name.split_once('_') {
                    if num.parse::<u8>().ok() == Some(track) {
                        track_dir = Some(entry.path());
                        break;
                    }
                }
            }
            let td = track_dir.with_context(|| format!("track {track} not found in {album}"))?;
            let stems_dir = td.join("stems");
            let work_dir = td.join("work");
            // Prefer new layout; fall back to legacy stems-everything layout.
            let tuned = if stems_dir.join("vocal.wav").exists() {
                stems_dir.join("vocal.wav")
            } else {
                stems_dir.join("tuned_vox.wav")
            };
            let target = if work_dir.join("f0_target.csv").exists() {
                work_dir.join("f0_target.csv")
            } else {
                stems_dir.join("f0_target.csv")
            };
            let report_dir = if work_dir.exists() {
                work_dir.clone()
            } else {
                stems_dir.clone()
            };
            fs_err::create_dir_all(&report_dir)?;
            if !tuned.exists() || !target.exists() {
                anyhow::bail!("no stems — run `nadir song render` first");
            }
            let rms = run_pitch_audit(&tuned, &target, &report_dir, None)?;
            println!("audit: {rms:.1} cents rms");
            Ok(())
        }
        SongSub::Listen {
            album,
            track,
            bed_preset,
            bpm,
        } => {
            let exe = std::env::current_exe()?;
            let mut args: Vec<String> = vec![
                "song".into(),
                "render".into(),
                "--album".into(),
                album.clone(),
                "--track".into(),
                track.to_string(),
            ];
            if let Some(bp) = bed_preset {
                args.push("--bed-preset".into());
                args.push(bp);
            }
            if let Some(b) = bpm {
                args.push("--bpm".into());
                args.push(b.to_string());
            }
            let status = std::process::Command::new(&exe).args(&args).status()?;
            if !status.success() {
                anyhow::bail!("song render failed");
            }
            // Locate render.wav
            let album_dir = std::path::Path::new("albums").join(&album);
            let mut found: Option<std::path::PathBuf> = None;
            for entry in fs_err::read_dir(&album_dir)? {
                let entry = entry?;
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if let Some((num, _)) = name.split_once('_') {
                    if num.parse::<u8>().ok() == Some(track) {
                        let new_render = entry.path().join(format!("{name}.wav"));
                        let legacy = entry.path().join("render.wav");
                        found = if new_render.exists() {
                            Some(new_render)
                        } else {
                            Some(legacy)
                        };
                        break;
                    }
                }
            }
            let r = found.context("render.wav not found after render")?;
            dispatch_play(&r)
        }
    }
}

fn run_pitch_audit(
    tuned_vox: &std::path::Path,
    f0_target_csv: &std::path::Path,
    work_dir: &std::path::Path,
    ceiling_cents: Option<f32>,
) -> Result<f32> {
    use nadir_feat::{extract_f0_lld, parse_f0_track, rms_cents_trimmed, SmileConfig};

    let smile_cfg = SmileConfig::default();
    let smile_csv = work_dir.join("opensmile_f0.csv");
    extract_f0_lld(&smile_cfg, tuned_vox, &smile_csv)?;
    let smile_text = fs_err::read_to_string(&smile_csv)?;
    let realized: Vec<(f32, f32)> = parse_f0_track(&smile_text)
        .into_iter()
        .filter(|(_, hz)| *hz > 0.0)
        .collect();
    let target_text = fs_err::read_to_string(f0_target_csv)?;
    let target: Vec<(f32, f32)> = target_text
        .lines()
        .skip(1)
        .filter_map(|l| {
            let mut it = l.split(',');
            let t: f32 = it.next()?.parse().ok()?;
            let h: f32 = it.next()?.parse().ok()?;
            Some((t, h))
        })
        .collect();

    // Align realized frames to nearest target frame by time.
    let mut realized_aligned = Vec::with_capacity(realized.len());
    let mut target_aligned = Vec::with_capacity(realized.len());
    for (t, r) in &realized {
        if let Some((_, tg)) = target
            .iter()
            .min_by(|a, b| (a.0 - *t).abs().partial_cmp(&(b.0 - *t).abs()).unwrap())
        {
            realized_aligned.push((*t, *r));
            target_aligned.push((*t, *tg));
        }
    }

    // Drop the worst 5% of frames (tracker transition glitches) before RMS —
    // closer to perceived tuning quality than straight RMS.
    let rms = rms_cents_trimmed(&realized_aligned, &target_aligned, 0.05);
    let ceiling = ceiling_cents.unwrap_or(30.0);
    let passed = rms <= ceiling;
    let report = serde_json::json!({
        "rms_cents": rms,
        "ceiling_cents": ceiling,
        "passed": passed,
        "n_realized_frames": realized.len(),
        "n_target_frames": target.len(),
    });
    fs_err::write(
        work_dir.join("audit.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    if !passed {
        tracing::warn!(
            rms_cents = rms,
            ceiling_cents = ceiling,
            "pitch audit above ceiling"
        );
    }
    Ok(rms)
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
                .args([
                    "run",
                    "--project",
                    "python/nadir-lyric-g2p",
                    "nadir-g2p",
                    "--stress",
                    "--voice",
                    &voice,
                    "--text",
                    &text,
                ])
                .output()
                .context("spawn uv for g2p")?;
            if !g2p_output.status.success() {
                anyhow::bail!(
                    "g2p failed: {}",
                    String::from_utf8_lossy(&g2p_output.stderr)
                );
            }
            // JSON: Vec<{phonemes:[str], stress:f32}>
            let word_data: Vec<serde_json::Value> =
                serde_json::from_slice(&g2p_output.stdout).context("parse g2p json")?;
            let phonemes_per_word: Vec<Vec<String>> = word_data
                .iter()
                .map(|v| {
                    v["phonemes"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|x| x.as_str().map(str::to_string))
                        .collect()
                })
                .collect();
            let stresses: Vec<f32> = word_data
                .iter()
                .map(|v| v["stress"].as_f64().unwrap_or(1.0) as f32)
                .collect();

            let syllables: Vec<String> = text.split_whitespace().map(str::to_string).collect();

            let k = Key::from_str(&key).map_err(|e| anyhow::anyhow!(e))?;
            let sk = ScaleKind::from_str(&scale).map_err(|e| anyhow::anyhow!(e))?;
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
                let f0_csv =
                    tempfile::NamedTempFile::with_suffix(".csv").context("create f0 csv")?;
                let f0_script = extract_f0_script(&current, f0_csv.path());
                run_inline(&praat_cfg, &f0_script, &[])?;
                let f0_text = fs_err::read_to_string(f0_csv.path())?;
                // Parse simple time_s,hz CSV (skip header)
                let realized: Vec<(f32, f32)> = f0_text
                    .lines()
                    .skip(1)
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
                let target_csv =
                    tempfile::NamedTempFile::with_suffix(".csv").context("create target csv")?;
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
                let snapped: Vec<(f32, f32)> =
                    realized.iter().map(|(t, hz)| (*t, sc.snap(*hz))).collect();
                let err_before: f32 = if realized.is_empty() {
                    0.0
                } else {
                    let sum: f32 = realized
                        .iter()
                        .zip(snapped.iter())
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
                let corrected =
                    tempfile::NamedTempFile::with_suffix(".wav").context("create corrected wav")?;
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
            let tmp = tempfile::NamedTempFile::with_suffix(".csv").context("create temp csv")?;
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
