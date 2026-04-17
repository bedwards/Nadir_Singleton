# 12 Memory Hymn — notes

## Why this slot
By track 12 the album has enough past to remember. We take a hymn form — slow, plain, affirmative — and use it to quote small phrases from earlier tracks: "light on the sill" (track 03), "a field a fence" (track 09), "a seed a thread" (track 10), "a name i did not say" (track 04). The listener recognises them without being told.

## Form and motif deployment
A dorian (the tonic of the album but brightened from Aeolian by the raised sixth), 64 bpm, 32 bars. `m.dawn_utterance` is deployed at the end of each four-bar phrase — eight times total. Each deployment is full: rising minor third, plateau held two bars. The hymn is the first track where the motif functions as a cadence rather than a gesture.

## csdr graph shape
Band-limit 80–3.2 kHz. We introduce a very-slow cross-fade between two noise beds: the bed from track 01 (shaped for dawn) and a brighter variant (shaped for late morning). Over 32 bars the balance crosses from 100:0 to 0:100. Memory is moving forward while the voice looks back.

## G2P / pronunciation hints
- "sill" matches track 03; "field" and "fence" match track 09; "seed" and "thread" match track 10. Use the same G2P transcriptions as the source tracks exactly.
- "before" as /b I f O r/, two syllables.
- "already" as /O l r E d i/, three syllables.
- "warm" as /w O r m/ — rhotic; avoid /w A r m/.

## openSMILE gate (primary)
Motif integrity: each of the eight `m.dawn_utterance` quotations must match the track-01 canonical template within 8 cents RMS on the plateau pitch. This gate asks openSMILE to confirm we have actually quoted ourselves.
