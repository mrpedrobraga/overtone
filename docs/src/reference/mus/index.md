# The `MUS` Standard

This standard consists of two halves:
- `.musx` (Musical Excerpt);
- `.musi` (Musical Instrument);

## Musical Excerpts (`.musx`)

Musical Excerpts is a data format for storing music (not audio), with notes, chords, and most importantly, markers.
Markers are arbitrary information that can be inscribed about a note, a group of notes, the pattern or the timeline.

There is no hard standard for what markers there can be. Instead, markers work through informal contracts.
An instrument developer might declare their "guitar plugin" can understand a `std::artificial_harmonics` marker...
Then, if you create a melody that uses a marker with this name anywhere, you know that specific instrument will understand.

Of course, it would be nice if developers can agree on how to call certain things: this way, the same musical data
can be used in many different instruments...

And thus, part of the `MUS` format is a [set of standard markers](./standard-markers) that your instruments should be able to recognise.

