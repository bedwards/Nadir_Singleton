# pitch_extract.praat — autocorrelation pitch track → CSV (time_s,hz)
form Pitch extract
    sentence InputWav
    sentence OutputCsv
    real TimeStep 0.01
    real PitchFloor 75
    real PitchCeiling 600
endform

Read from file: inputWav$
To Pitch (ac): timeStep, pitchFloor, 15, "no", 0.03, 0.45, 0.01, 0.35, 0.14, pitchCeiling

nframes = Get number of frames
deleteFile: outputCsv$
appendFileLine: outputCsv$, "time_s,hz"
for i from 1 to nframes
    t = Get time from frame number: i
    f = Get value in frame: i, "Hertz"
    if f = undefined
        f = 0
    endif
    appendFileLine: outputCsv$, fixed$(t, 6), ",", fixed$(f, 4)
endfor
