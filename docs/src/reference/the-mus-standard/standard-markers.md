# Standard Markers

For the benefit of the `MUS` community, here is a relation of standard markers your instruments should follow.
This doesn't mean every instrument should do something for _every_ marker, but if it has any of these functionalities,
you should prioritize using one of these names before creating your own.

#### A short aside on naming convention

The names included were chosen to be as general and applicable as possible, while still remaining understandable. Unlike a program like _MuseScore Studio_, which has a focus on the aesthetics of notation (and thus has many redundant ways of notating the same concept), we prioritise high interchangeability instead of specificity.

Then, it's up to the GUI that's editing a `.musx` file to render the notation appropriately according to the user choice and the current instrument. For example, a classical composer might prefer to read `pitch_slide` as "**Portamento**", while an EDM composer might prefer "**Slide**".

Lastly, it's convention to "namespace" strings to the standard that defined them, such that you know exactly what piece of technical information someone is refering to. So, if a marker here is called `vibrato` it'll be referred to as `std:vibrato`, as opposed to some third party's marker that would be called `thirdparty:vibrato` and could potentially work differently;

### Note markers

Markers that modify or add effects to a single note or a set of notes.

> [!TIP]
> All markers that affect "a single note" can be applied to multiple notes at once, but some markers only make sense when applied to several notes, like legato.

#### Articulations
- `accent` for heighted accent on an note;
    - **(a.k.a. Marcato)**;
- `legato` for notes that are played together;
    - **(a.k.a. Legato)**;
- `staccato` for notes that are not held for their full time;
    - **(a.k.a. Staccato)**; 
- `vibrato` for quickly sliding a note up and down;
    - **(a.k.a. Vibrato)**;
- `tremolo` for quickly sliding a note's volume up and down;
    - **(a.k.a. Tremolo)**;
- `harmonics` for natural and artificial harmonics;
    - **(a.k.a. harmonics)**;
- `method` for changing the tool and overall method used to emit sound;
    - Has a single `String` parameter, which can be:
        - `"default"`, for the default method of the MUSi;
        - `"plucking"`, useful for string instruments;
            - **(Plucked, pizz.)**;
        - `"pick"`, useful for the guitar family;
            - **(Picked)**;
        - `"bow"`, useful for the violin family;
            - **(Arco)**;
        - `"soft_mallet"`, useful for percussion;
        - `"hard_mallet"`, useful for percussion;
        - Or something else;

#### Pitch
- `pitch_shift` for shifting the pitch of a selected section by some amount;
    - **(a.k.a. ♯, ♭, 8va. Alta & Bassa, etc.)**;
- `pitch_slide ( Mode )` for gradually shifting pitch between two selected notes;
    - **(Glissando)**;
    - `continuous` **(portamento)**;
    - `stepped` **(glissando, chromatic)**;
    - `diatonic` **(glissando, in key)**;
    - `discrete` **(hammer on & pull off)**;
- `sustain` for allowing notes to vibrate after they are released;
    - **(a.k.a. Ped., let ring, laissez vibrez)**

#### Dynamics
- `intensity` for setting the intensity with which a note is triggered.

#### Ornaments
- `chord_derive` for deriving a chord from a single note. Useful for sketching harmony non-destructively;
- `chord_stagger` for arpeggiating/strumming a chord;
    - **(a.k.a. Arpeggio, Strum)**;
- `trill` for quickly arpeggiating a note up and down;
    - **(a.k.a. Trill)**;
- `lyrics` for adding lyrics to a certain note;

### Voice markers

Markers that affect the song itself, at least for a specific voice.

- `time_resubdivide ( Int )` for subdividing a stretch of time in a new amount of parts;
    - **(a.k.a. tuplets)**;

### Timeline markers

Markers that are affect or are bound to the timeline.

These markers _are_ seen by MUSI during playback, but most instruments aren't supposed to react to the tempo anyways. Instead, these are to be interpreted and played back by the MUSI's host.

#### Location
- `marker` a simple marker that simply marks a place or range in the song.

#### Song pace
- `tempo_define`, for defining a new pace for the song, in units (beats) per second;
- `tempo_slide` for accelerating or decelerating the tempo;
    - **(accel., rit., etc.)**;
- `tempo_divide`, for setting a new scheme for dividing the flowing tempo (what's called a 'time signature');
- `tempo_feel`, for defining a new tempo style;
    - Has a single parameter, which can be:
        - `"straight"` for **Straight** tempo;
        - `"swing"` for **Swing** tempo;
        - Or something else;
- `repeat` for repeating an area of the song some amount of times;
- `conditional_jump` for jumping to some specific marker given an input condition. This one is advanced and might be used for repeats or dynamic, interactive music;