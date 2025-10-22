# The MUSX (Musical Excerpt) format

An excerpt of musical data that will be played
by a MUSi (Musical Instrument) or another MUS-compliant plugin.

This format contains a bunch of notes in a sorted buffer,
such that they can be seeked efficiently using binary search.

Each note has eight inherent parameters:
- `start`;
  - the time offset for this note from the beginning of the song;
- `duration`;
  - how long the note will last for;
- `voice`;
  - the voice that plays this note, for instruments with multiple voices;
- `intensity`;
  - the intensity with which the note is played (a.k.a. velocity);
- `pitch`;
  - the pitch of the note;
- `kind`;
  - the "kind" of the note, for instruments with many emitters (like percussion) or articulations (like strings);
- `sample_variant`;
  - the sample variant, which can be used to add variation to a song;
- `extra`;
  - an extra parameter that you can use for whatever;

It additionally contains "notation" — time-bound and note-bound markers,
each which add additional information to be interpreted by the MUSi.
These markers are also arbitrary. They may look something like this:

`std::staccato`, `std::chord_stagger`, `std::artificial_harmonics`.

## Playback

When you play a song in Overtone, the play head scans the notes from left to right and emits note events in real time.
This is how MUSi plays excerpts, and also how it works if you're recording an excerpt using a MIDI controller.

But the instruments don't need to react "in real time" to events, since they do get a view into the
currently-playing excerpt and are able to _look ahead into the future_.

This might be useful for certain instruments — the ability to generate sounds for blocks of music instead of in a 
note-by-note basis.