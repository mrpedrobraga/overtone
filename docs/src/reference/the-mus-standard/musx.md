# The MUSX (Musical Excerpt) format

An excerpt of musical data that will be played
by a MUSi (Musical Instrument) or another MUS-compliant plugin.

This format contains a bunch of notes in a sorted buffer,
such that they can be seeked efficiently using binary search.
Each note has

It additionally contains two kinds of markers — time-bound and note-bound markers,
each which add additional notation to be interpreted by the MUSi.
These markers are also arbitrary. They may look something like this:

`std::staccato`, `std::chord_stagger`, `std::artificial_harmonics`.
