# What defines music — a first-principles essay

This essay answers a narrow question: what is the minimum set of properties a stream of sound must possess for a human listener to hear it as music, rather than as noise, signal, or environment? The answer must be tight enough to serve as a design contract for Nadir_Singleton, which renders every track through only five tools — MBROLA, Praat PSOLA, openSMILE, Silero-VAD, and csdr. No synths. No sample libraries. No neural vocoders. If we cannot state what music is in terms those tools can realise, the project is undisciplined. If we state it too broadly, anything the pipeline emits counts; the constraint collapses. We want the opposite: a definition sharp enough to refuse most outputs and to shape the rest.

## Physical substrate

Sound is a longitudinal pressure wave in an elastic medium, typically air. At the level of physics it carries only three degrees of freedom that matter to a listener: instantaneous amplitude, instantaneous frequency content, and the time structure over which both evolve. Everything a microphone captures and everything an eardrum transduces reduces to a one-dimensional scalar function p(t) of pressure over time. All acoustic richness is encoded in that single function.

The waveform itself is, however, the wrong level of description for music. Two pressure signals that are mathematically distinct can be perceptually identical — phase scrambling below a few hundred Hertz is largely inaudible within a critical band, and two tones differing only in their starting phase produce indistinguishable percepts once steady-state. Conversely, two waveforms with near-identical RMS envelopes can sound like utterly different things because their *spectral content* differs. The cochlea is a mechanical Fourier analyser: the basilar membrane separates frequency components by place, and the auditory nerve fires at rates proportional to both the amplitude at each place and (up to about 4–5 kHz) the phase-locked temporal fine structure of each component. The brain therefore receives something much closer to a running short-time spectrogram than a raw waveform.

Consequences for a production system: (1) spectral structure is the primary design surface, not waveform morphology; (2) the time-frequency tradeoff (Gabor/Heisenberg limit) is not a nuisance but a constraint musicians work *within*; (3) manipulations that preserve spectral envelope while altering fine structure (PSOLA, phase vocoder) largely preserve perceptual identity, which is precisely why Praat PSOLA is safe for melodic transposition of vocal material. Our tools are spectral-first by construction.

## Psychoacoustic foundations

Before sound becomes music, it is filtered by the peripheral auditory system. The key empirical results are not optional — they are the physical substrate of musical listening and any definition of music must be compatible with them.

**Equal-loudness contours.** Fletcher and Munson (1933), later refined by Robinson–Dadson and standardised as ISO 226, showed that perceived loudness is a strongly frequency-dependent function of SPL. A 60 dB tone at 3 kHz is far louder than a 60 dB tone at 60 Hz. Any compositional intuition about "balance" between parts is implicitly a claim about equal-loudness weighting at the mix-bus level. A production system that mixes by RMS without weighting will systematically under-balance low-register material.

**Critical bands and the Bark scale.** Zwicker's Bark scale partitions the spectrum into roughly 24 critical bands corresponding to ~1.3 mm segments along the basilar membrane. Energy within a critical band is integrated non-linearly; energy in separate bands is largely independent. This single fact explains consonance roughness (Plomp & Levelt), simultaneous masking, and why thirds sound rougher in the low register (partials fall in the same critical band) than in the middle register (they separate). For Nadir_Singleton it means: if two pitched elements are within ~1 Bark of each other, they fuse or beat; if they are more than ~1 Bark apart they stream separately.

**Masking.** Simultaneous masking (louder components hide quieter components within the same critical band) and temporal masking (forward masking ~100–200 ms, backward masking ~20 ms) set the perceptual floor of a mix. Any element below the masking threshold does not exist for the listener. This is why a busy arrangement must be built by carving spectral and temporal holes, not by layering. csdr's gated envelopes and spectral filtering exist precisely for this.

**Pitch perception.** Two theories coexist and both are correct in their domains. Place theory (Helmholtz, Békésy) handles pitch above ~5 kHz and accounts for the tonotopic map. Temporal theory (phase-locked firing patterns) handles pitch below ~5 kHz and explains how pitch survives spectral distortion. The reconciliation is Terhardt's virtual pitch model: the auditory system extracts a *residue pitch* from the pattern of harmonics, even when the fundamental is absent. A telephone reproduces no energy below 300 Hz yet voices retain their pitch, because the harmonic series (400, 500, 600, 700 Hz for a 100 Hz fundamental) unambiguously implies the missing f0. This is load-bearing for our pipeline: MBROLA's diphone output and any Praat-resynthesised vocal can be spectrally shaped aggressively without pitch being destroyed, as long as enough harmonics survive.

**Pitch salience is graded, not binary.** Terhardt distinguishes *spectral pitch* (a pure tone at a given frequency) from *virtual pitch* (the implied root of a harmonic complex). Musical pitch is almost always virtual pitch. This is why a sawtooth and a square wave at 220 Hz are heard as the same pitch despite having radically different spectra, and why non-harmonic sounds (bells, gongs) have ambiguous pitch — the residue pitch extractor has no single best candidate.

## Cognitive primitives

Peripheral hearing delivers a running time-frequency representation. Cognition turns it into music through a small set of organising operations, each of which is well-characterised and each of which must be respected by the compositional grammar.

**Auditory Scene Analysis.** Bregman's 1990 synthesis established that the auditory system solves a scene-parsing problem continuously: it groups spectral energy into *streams* that each represent a single sound source. Grouping cues are proximity in frequency, synchrony in onset/offset, common amplitude modulation, spatial co-location, and harmonic relationship. Any component that obeys these cues fuses; any component that violates them segregates. Music exploits this at every scale: a chord is a single stream only because its partials share onset and harmonic relation; a melody is a stream because successive notes are proximate in frequency and regular in time; two instruments playing together are *meant* to be heard as two streams, which is why unison doubling is a special effect and not a default.

**Streaming and the galloping rhythm.** Van Noorden's experiments with alternating high-low tones (A-B-A-B) show that as either tempo or frequency separation increases, listeners flip from hearing one galloping stream to hearing two interleaved streams. This defines what a composer can and cannot do with register and tempo: wide-leap fast lines cannot be heard as a single melody, no matter how they are notated.

**Rhythm as periodicity detection.** Humans entrain motor and neural oscillators to periodic acoustic events in a narrow range — roughly 0.3 to 3 Hz, with a preferred tempo around 1.5–2 Hz (90–120 BPM). Below this range there is no pulse; above it, events become texture. Meter is a hierarchical grouping of pulses (typically binary or ternary), which the brain imposes even on isochronous stimuli (the *tick-tock illusion*). Rhythm is not the stimulus; rhythm is the listener's inference *from* the stimulus, constrained by the stimulus's periodicity structure.

**Pitch as relation.** The fact of absolute pitch is rare and largely irrelevant. Musical pitch is heard relationally: a melody transposed across registers is recognised as the same melody, because the relevant quantity is the interval pattern. This is why scale systems matter more than absolute frequency. It is also why a pipeline that guarantees bounded scale membership will produce something hearable as "a tune" even with crude timbres.

**Form as memory and expectation.** Huron's ITPRA model (Imagination, Tension, Prediction, Reaction, Appraisal) formalises what Meyer argued in 1956: musical experience is the interplay of learned statistical expectations and their confirmation, delay, or violation. A listener tracks repetition, return, and deviation on a timescale of seconds to minutes. Without return, there is no expectation. Without expectation, there is no music — only sound events. This is the single most important constraint on generative systems and the one most often violated.

**Lerdahl & Jackendoff's GTTM** operationalises tonal music parsing as a set of preference rules for grouping, metric, time-span reduction, and prolongation. Their framework is strong evidence that tonal form is structurally parseable in the way language is, and that well-formed music exhibits nested hierarchical structure — phrases within sections within movements.

## Cross-cultural invariants

Ethnomusicology resists universals on principle, but empirical surveys — Lomax's cantometrics, Savage et al.'s 2015 statistical cross-cultural study of 304 recordings — converge on a small set of features that appear in every documented human musical tradition. Patel's *Music, Language, and the Brain* synthesises these.

**Universally present:**
- Discrete pitched material organised into scales with fewer than ~10 pitches per octave. No known tradition uses continuous-pitch-only music for its central repertoire; even vocal traditions with heavy ornament (Indian classical, Qawwali, Tuvan throat-singing) articulate structural scale degrees.
- Periodic or quasi-periodic rhythmic organisation. Isochronous pulse is dominant but not universal; what is universal is *some* temporal regularity that supports entrainment.
- Repetition at the phrase, section, and piece scale. Strophic forms, variation forms, call-and-response, and motif-and-development all instantiate the same cognitive requirement: return enables prediction.
- Use of the chest/head voice as primary melodic instrument, or instruments that imitate it.
- Octave equivalence in scale construction. Pitches separated by 2:1 are treated as the same pitch class.

**Variable across traditions:**
- Which scales (12-TET is a recent, geographically narrow default; just intonation, N-TET systems, maqam with microtonal intervals, slendro/pelog with non-octave-equal steps all exist).
- Whether harmony in the Western functional sense is present (it is geographically rare).
- Timbral valuation: "beautiful" timbre is culture-specific. Bulgarian choral timbre, shakuhachi breath noise, overdriven electric guitar — none is universally consonant.
- Notation or its absence.
- The role and duration of silence.

For our purposes the invariants matter: pitched material in a scale, periodic event grid, repetition/return. The variables give us freedom: we can choose our scale, our timbral palette, and our silence economy without breaking the contract with the listener's cognitive machinery.

## Intent and listener contract

None of the above fully characterises music. Bird song has pitch, rhythm, and repetition. A washing machine's spin cycle has pitched material, periodicity, and near-repetition. The distinguishing feature is twofold: *intentional temporal organisation* on the production side, and a *musical listening stance* on the reception side.

Scruton and Levinson, from different positions, converge on this: music is sound organised with the intent that it be heard as music, and received with the expectation that it rewards musical attention. This is circular only at first glance. The circularity dissolves because both sides ground out in the cognitive primitives already named: the producer arranges sound to engage grouping, pitch-relation, and return; the listener deploys those faculties and checks whether the sound rewards them.

This explains edge cases. Noise art (Merzbow, early Cage tape work) qualifies as music when the organisation of spectral energy across time rewards structured listening — changes in density, return of spectral regions, articulated phases. It fails to qualify when it is simply a continuous wall with no temporal shaping. Random sine tones do not qualify because they afford neither streaming (independent onsets by design) nor prediction (stochastic by design), even though each tone is pitched. Aeolian harps qualify intermittently, when wind produces passages of organisation, and lapse into non-music when they don't.

The listener contract is the reason for LINER.md, track titles, and the album framing in Nadir_Singleton. Framing a stream of audio as "track 3 of an album" primes the listener into musical stance. The same audio presented as "data sonification of CPU load" cues a different stance and will be parsed differently by the same brain.

## Defining music operationally

Music is intentional temporal organisation of spectral energy that affords perceptual grouping (rhythm), pitch relation (melody/harmony), and return (form) under a musical-listening contract.

That sentence is the design target. Each clause decomposes:

- *intentional* — a producer chose these events; randomness without curation fails.
- *temporal organisation* — the arrangement extends through time with structure at multiple scales.
- *spectral energy* — not waveform-level design, but time-frequency design.
- *perceptual grouping (rhythm)* — events must cluster into streams and pulse.
- *pitch relation (melody/harmony)* — pitches must relate to other pitches, not float in absolute space.
- *return (form)* — something must come back, at some scale.
- *musical-listening contract* — the output is framed and received as music.

Every engineering decision below is justified by one of these clauses.

## Minimum viable music — a reduction

For Nadir_Singleton, the minimum sufficient set of phenomena a rendered stem must display to count as music is:

1. **Bounded pitched material in a scale.** At least one stream in the stem carries a sequence of pitches drawn from a named scale (12-TET, JI, N-TET, maqam, etc.), with fewer than ~10 pitches per octave, and respects octave equivalence. Pitch may be ornamented but must return to scale degrees.
2. **Periodic or quasi-periodic event grid.** Onsets of at least one stream align to a tempo grid between 30 and 240 BPM (one full octave of tactus), with metrical depth ≥ 2 (i.e., some hierarchy: pulse + measure). Jitter is permitted; isochrony is not required; detectable periodicity is.
3. **At least one element of repetition or return over the duration.** A motif, a section, a rhythmic cell, or a timbral gesture must recur in recognisable form at least once after its introduction. Through-composed is permitted only if it recalls a motif.

These three are individually necessary and jointly sufficient within our pipeline. A stem satisfying all three, rendered by the 5-tool chain, and framed in an album context, counts as music under the operational definition above. A stem missing any of the three does not.

## Implications for Nadir_Singleton design

Each operational clause maps to specific tools and specific module responsibilities.

**Bounded pitched material → MBROLA F0 targets, Praat PSOLA snap, openSMILE verification.**
MBROLA's diphone synthesis accepts explicit F0 values at phoneme boundaries. The compose module emits F0 sequences quantised to the track's scale. Praat PSOLA is used post-hoc for fine pitch correction and for scale-degree snap when MBROLA's internal interpolation has drifted. openSMILE extracts F0 contours from the rendered audio; the analysis module compares extracted F0 to the target scale and flags any stem where more than N% of voiced frames lie outside the scale by more than ±25 cents. This is the closed loop that enforces clause (1).

**Periodic event grid → Silero-VAD onsets, BPM grid, csdr gated envelopes.**
Silero-VAD provides voice-activity onsets with sub-100ms resolution. For vocal streams these onsets are the ground truth of rhythm. The compose module authors a BPM and a metrical template; MBROLA segment durations are computed so that syllable onsets fall on grid positions (with permitted micro-timing offsets). For non-vocal textures, csdr's gated envelopes are driven by the same grid via LFO-style modulators so that spectral events share the pulse. The analysis module verifies by running Silero-VAD on the final stem and measuring the autocorrelation peak of the onset sequence at the intended BPM.

**Return / form → compose module's form grammar, cross-album motif catalog.**
The compose module operates over a finite set of form grammars: AABA, verse/chorus, rondo (ABACA), through-composed-with-motif-recall, theme-and-variations, and binary. Each grammar is a template that repeats a *motif* (melodic, rhythmic, or spectral). A cross-album motif catalog persists motifs across tracks so that intra-album recall is possible at the album scale, not only the track scale. This is what makes the whole an album and not an assemblage.

**Spectral energy → csdr pipelines, Praat-generated textures, MBROLA timbre.**
csdr supplies the DSP primitives: filters, shifters, compressors, modulators, additive and subtractive operations on IQ or real signals. Praat's Sound manipulation generates textural material (filtered noise, formant-shifted vocals). MBROLA's voice selection (DE7, US1, FR1, etc.) provides timbral variety at the vocal layer. The spectral palette of a track is specified as a csdr pipeline graph plus a Praat recipe plus an MBROLA voice, all committed to the track's recipe.

**Listener contract → LINER.md + track titles + narrative arc.**
Each track carries a title, a position in an album, and a note in LINER.md describing stance. The album as a whole presents narrative arc — an order, a direction, a resolution. This framing cues musical listening on the receiver side, closing the contract.

## Why this constraint set yields *music* and not *sonification*

Sonification is the rendering of data as sound without compositional shaping: the sound is a transparent window onto the data. Our pipeline could easily slide into sonification — openSMILE emits feature vectors, and the temptation is to map them directly to audio. We refuse this for reasons grounded in the definition above.

Vocals as the primary carrier force the output through a channel the listener's brain is *biologically specialised for*. Speech is the single auditory signal humans parse with the most dedicated neural machinery; piggybacking musical structure onto speech-like signal guarantees that grouping, streaming, and prosodic tracking engage. MBROLA's diphones, even when pitched onto scale degrees and stretched onto a grid, remain phonetically legible enough that the listener's speech-perception circuits run. Music that exploits this — song, chant, sprechgesang — is perceptually dense in a way that pure instrumental sonification is not.

Scale-tuning ensures that pitch relation is present, not merely pitch. Two pitches drawn from a scale carry interval information; two arbitrary pitches do not. The listener's scale-schema (Krumhansl's tonal hierarchy) activates and begins predicting. This is musical cognition engaging, not signal cognition.

Narrative form (AABA, verse/chorus, etc.) introduces return. Return is what flips the listener from "observing a signal" to "tracking a piece." ITPRA requires prior exposures to fire; without return, no prior exposure is available within the piece, and the listener defaults to exploratory or environmental listening.

In combination, vocals + scale + form route the output away from sonification territory. A stem with openSMILE-derived pitch contours, unscaled, unvocal, and non-repeating would be sonification. The same pipeline with scale-quantisation, vocal rendering, and form-grammar output is music. The tools do not change; the compositional constraints do.

## Scales and tunings we permit

Default is 12-TET. It is the lingua franca of Western-trained listeners and of every playback system. Tracks committed to 12-TET require no listener adaptation and mix trivially with one another.

Just intonation is permitted per-track, declared in the recipe. JI is particularly productive with MBROLA material because vocal harmonics are naturally harmonic-series-aligned and beat against tempered thirds and sevenths; JI removes the beating and produces a purer stack at the cost of modulation capability. A JI track commits to a key.

N-TET (19-TET, 22-TET, 31-TET) is permitted per-track. These systems give alternate consonance structures and are realisable trivially via Praat PSOLA's arbitrary F0 targeting. A 31-TET track will sound unfamiliar but coherent; a quarter-tone (24-TET) track will sound middle-eastern-inflected; 19-TET will sound like a minor-third-heavy tempered system.

Microtonal ornament (pitch bends, glides, portamento) is permitted as decoration around scale degrees, not as the material itself. Praat PSOLA can produce arbitrarily fine F0 contours; we restrict its use so that structural pitch always lands on scale degrees.

Continuous pitch glides without destination (trombone-like smears that never resolve, synthesis with no pitch target) are discouraged. They violate the bounded-pitched-material clause and push toward sonification.

## What we reject

- **Sonification of openSMILE vectors with no compositional shaping.** Feature vectors piped directly to oscillators or filter parameters without form grammar, scale quantisation, or motif logic. No matter how the audio sounds, it fails the definition.
- **Pure noise with no return.** Any texture, however spectrally rich, that never recalls itself. Noise art with articulated returns is fine; uniform noise is not.
- **Generative that never repeats.** Stochastic processes that produce novel material indefinitely and never cycle. They violate clause (3).
- **Generative without any pitched material.** Rhythmic and textural work with no scale-bounded pitched stream. Percussion-only stems may exist within a track but a track that is percussion-only, with no pitched layer, fails clause (1).
- **Random walks in pitch space.** Sequences of pitches that do not commit to a scale, even if locally consonant. The listener's tonal-hierarchy machinery cannot lock in.
- **Sub-tactus or super-tactus event streams as primary rhythm.** Streams whose IOI is below 200 ms (>5 Hz, texture territory) or above 2 s (<0.5 Hz, sub-pulse territory) as the sole rhythmic layer. These can occur as secondary layers.

## Working definition (final)

Music is intentional temporal organisation of spectral energy that affords perceptual grouping (rhythm), pitch relation (melody/harmony), and return (form) under a musical-listening contract.

For Nadir_Singleton: a rendered stem counts as music iff (a) it contains at least one stream of pitched material bounded to a declared scale, (b) its events align to a declared tempo grid between 30 and 240 BPM with metrical depth ≥ 2, and (c) a motif, section, or gesture recurs at least once after its introduction, the whole framed by album context.

## Reading list

- Albert S. Bregman, *Auditory Scene Analysis: The Perceptual Organization of Sound* (MIT Press, 1990). The foundational text on streaming, grouping, and scene parsing.
- David Huron, *Sweet Anticipation: Music and the Psychology of Expectation* (MIT Press, 2006). Source of the ITPRA model; the definitive account of expectation-driven musical experience.
- Aniruddh Patel, *Music, Language, and the Brain* (Oxford, 2008). Comparative cognition of music and language; strong on cross-cultural invariants.
- Fred Lerdahl and Ray Jackendoff, *A Generative Theory of Tonal Music* (MIT Press, 1983). Hierarchical parsing of tonal form; the closest thing to a formal grammar of Western tonal music.
- Ernst Terhardt, "Pitch, consonance, and harmony" (JASA, 1974) and subsequent papers on virtual pitch. The definitive treatment of residue pitch and harmonic-series-derived pitch perception.
- Harvey Fletcher and Wilden Munson, "Loudness, its definition, measurement and calculation" (JASA, 1933). The original equal-loudness work; foundation of all loudness weighting.
- Eberhard Zwicker and Hugo Fastl, *Psychoacoustics: Facts and Models* (Springer, 3rd ed. 2007). Reference work on critical bands, masking, and loudness models.
- Diana Deutsch (ed.), *The Psychology of Music* (Academic Press, 3rd ed. 2013). Authoritative survey covering pitch, rhythm, scale, and memory.
- Carol Krumhansl, *Cognitive Foundations of Musical Pitch* (Oxford, 1990). Empirical basis for tonal hierarchy and key-finding.
- Leonard Meyer, *Emotion and Meaning in Music* (Chicago, 1956). The original statement of expectation-as-musical-meaning; Huron's antecedent.
- Patrick Savage et al., "Statistical universals reveal the structures and functions of human music" (PNAS, 2015). Empirical cross-cultural confirmation of the invariants cited above.
