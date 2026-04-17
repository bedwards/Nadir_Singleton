# psola_retarget.praat — resynthesize input WAV with F0 contour from CSV (time_s,hz)
form PSOLA retarget
    sentence InputWav
    sentence PitchCsv
    sentence OutputWav
endform

Read from file: inputWav$
sound$ = selected$("Sound")
To Manipulation: 0.01, 75, 600

Create PitchTier: "target", 0, 1
table = Read Table from comma-separated file: pitchCsv$
nrow = Get number of rows
for i from 1 to nrow
    selectObject: table
    t = Get value: i, "time_s"
    h = Get value: i, "hz"
    selectObject: "PitchTier target"
    Add point: t, h
endfor

selectObject: "Manipulation " + sound$
plusObject: "PitchTier target"
Replace pitch tier

selectObject: "Manipulation " + sound$
resyn = Get resynthesis (overlap-add)
selectObject: resyn
Save as WAV file: outputWav$
