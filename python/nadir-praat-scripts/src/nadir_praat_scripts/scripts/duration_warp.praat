# duration_warp.praat — time-warp input by ratio using PSOLA
form Duration warp
    sentence InputWav
    sentence OutputWav
    real Ratio 1.0
endform

Read from file: inputWav$
sound$ = selected$("Sound")
To Manipulation: 0.01, 75, 600
dur = Create DurationTier: "dur", 0, 10
Add point: 0, ratio
Add point: 10, ratio
selectObject: "Manipulation " + sound$
plusObject: dur
Replace duration tier
selectObject: "Manipulation " + sound$
resyn = Get resynthesis (overlap-add)
selectObject: resyn
Save as WAV file: outputWav$
