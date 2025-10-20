# Standard Markers

For the benefit of the `MUS` community, here is a relation of standard markers your instruments should follow.
This doesn't mean every instrument should do something for _every_ marker, but if it has any of these functionalities,
you should prioritise using one of these names before creating your own.

### Note markers

Markers that modify or add effects to a single note or a set of notes.

- `accent` for heighted accent on an note;
    - **(Marcato)**;
- `group` for notes that are played together;
    - **(Legato)**;
- `short` for notes that are not held for their full time;
    - **(Staccato)**; 
- `stagger` for arpeggiating/strumming a chord;
    - **(Arpeggio)**;
- `trill` for quickly arpeggiating a note up and down;
    - **(Trill)**;
- `vibrato` for quickly sliding a note up and down;
    - **(Vibrato)**;
- `tremolo` for quickly sliding a note's volume up and down;
    - **(Tremolo)**;
- `pitch_shift` for shifting the pitch of a selected section by some amount
    - **(Sharps and Flats; Ottava Alta & Bassa, etc.;)**;
- `pitch_slide ( Mode )` for gradually shifting pitch between two selected notes;
    - **(Glissando)**;
    - `continuous` **(portamento)**;
    - `stepped` **(glissando, chromatic)**;
    - `diatonic` **(glissando, in key)**;
    - `discrete` **(hammer on & pull off)**;
- `sustain` for allowing notes to vibrate after they are released;

### Voice markers
- `time_subdivide ( Int )` for subdividing a stretch of time in a new amount of parts;
    - **(Tuplets)**;
- `sound_tool ( String )` for changing the tool used to emit sound;
    - `fingers`, useful for string instruments;
        - **(Plucked, Pizz.)**;
    - `pick`, useful for the guitar family;
        - **(Picked)**;
    - `bow`, useful for the violin family;
        - **(Arco)**;
    - `soft_mallet`, useful for percussion;
    - `hard_mallet`, useful for percussion;

### Timeline markers
- `tempo_define ( Number )`, for defining a new tempo;
- `tempo_divide ( Option<TimeSignature> )`;
    - **(Time signatures)**;
- `tempo_feel ( String )`, for defining a new tempo style, such as:
    - `"straight"` for **Straight** tempo;
    - `"swing"` for **Swing** tempo;
- `tempo_slide ( Number )`, for accelerating or decelerating the tempo;
    - **(Accel., Rit., etc.)**;
