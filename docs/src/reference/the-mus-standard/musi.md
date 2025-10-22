# MUSI (Musical Instruments)

A simple sample library, similar to SoundFont, that can generate audio from music
using a format for arbitrary semantic music exchange (MUSX).

Each instruments carries:
- Samples, each annotated with parameters;
- A Sampling Scheme, which describes how to fulfill the request to play some note;

Then, in a host like Overtone, it can be driven to generate audio in either real time or offline.