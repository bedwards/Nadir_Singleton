# formant_shift.praat — LPC-based formant frequency shift
form Formant shift
    sentence InputWav
    sentence OutputWav
    real ShiftRatio 1.0
    real NewPitchMedian 0.0
endform

Read from file: inputWav$
Change gender: 75, 600, shiftRatio, newPitchMedian, 1.0, 1.0
Save as WAV file: outputWav$
