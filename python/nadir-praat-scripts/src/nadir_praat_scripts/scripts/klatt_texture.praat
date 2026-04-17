# klatt_texture.praat — generate a KlattGrid-driven texture clip
form Klatt texture
    sentence OutputWav
    real Duration 4.0
    real F0 110
endform

Create KlattGrid: "tex", 0, duration, 6, 1, 1, 6, 0, 0, 0
Add pitch point: 0, f0
Add pitch point: duration, f0 * 1.02
Add voicing amplitude point: 0, 90
Add formant frequency point: 1, 0, 700
Add formant frequency point: 1, duration, 620
Add formant frequency point: 2, 0, 1200
Add formant frequency point: 2, duration, 1400
Add formant frequency point: 3, 0, 2500
Add formant frequency point: 3, duration, 2500
To Sound
Save as WAV file: outputWav$
