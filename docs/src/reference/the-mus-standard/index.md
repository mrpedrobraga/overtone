# The `MUS` Standard

This standard consists of two halves:
- `.musx` (Musical Excerpt);
- `.musi` (Musical Instrument);

## Musical Excerpts (`.musx`)

Musical Excerpts is a data format for storing music (not audio), with notes, chords, and most importantly, markers.
Markers are arbitrary information that can be inscribed about a note, a group of notes, the pattern or the timeline.

There is no hard standard for what markers there can be. Instead, markers work through informal contracts.
An instrument developer might declare their "guitar plugin" can understand a `std:harmonics` marker...
Then, if you create a melody that uses a marker with this name anywhere, you know that specific instrument will understand.

[Read More ->](./musx)

## Musical Instruments (`.musi`)

MUSI is a data format for defining sample libraries that can play back musical excerpts (MUSX)
or data transferred through the MUSP protocol.

Each MUSI stores several samples annotated with numerical parameters.
When the instrument is asked to play a note, it will choose the most suitable sample to play
taking into account the requested note's parameters, the markers applied to it and the previous markers.

This way, MUSI can react to articulations and ornaments and all the like.

[Read More ->](./musi)