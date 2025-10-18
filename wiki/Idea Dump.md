## Stability & Robustness
- Lazy-load everything;
- Continuously save operations to disk — don't wait for
- `Task<T>` for running concurrent background tasks;
- Run third party plugins as processes in sister threads so if they crash they don't take the whole program with itself;
- 
## The Editor API
- Interface for interacting indirectly with projects;
	- Multiple clients editing concurrently;
	- Remote editing over a network;
	- Command Palette — execute actions by name;
	- 
## Production Setup
- Graph View for composing data through effects through plugins;
	- Connect the cables using cool curves;
- Plugin GUI might be animated while in use;
	- Guitar strings will be plucked;
	- Piano keys will be pressed (and pedals will be held);
- Live Preview of stuff;
	- Audio Preview;
	- Lyrics Preview;
	- Piano Roll (MIDI-like) Preview, like Synthesia;
![[concept_production_setup.png]]

## Piano Roll
- Cool piano roll that renders notes as well as musical markings and directives;
	- Pitch View, viewing pitches over time;
	- Voice View, viewing all voices side by side. Useful for choir or string instruments;

## Signal Types
- Music (`Overtone MUS`);
- MIDI;
- Audio (`PCM 16 bits, N channels`);
- Lyrics;
- Number;
- Device (as in an audio device);

## Effects
- Audio;
	- Gain / Volume Scale;
	- Equalisation / Frequency-space;
- Music;
	- **Staccato**: Make all notes short notes;
	- **Note shift**: Shift all the notes' pitches by some amount. Note that this operation happens music-wise and not audio-wise, and thus will produce more accurate timbres;
	- **Arpeggio**: Transforms polyphonic music into monophonic signals by arpeggiating chords;

## Inputs
- Audio;
	- Microphones;
	- System audio, and anything supported by CPAL;
- MIDI;
	- MIDI Controllers;
- Music;
	- Some plugins in Overtone;
- Number;
	- Knobs, sliders, edits, toggles;

## Sinks
- Audio;
	- Audio Preview (on Device);
	- Export to file (`WAV`, `OGG`, `MP3`, `QOA`);
	- Frequency Analyser;
	- Wave Viewer;
- Music;
	- Music Visualiser;
	- Export to file (`Overtone MUS`, `MID`, `MusicXML`);
- Lyrics;
	- Lyrics Visualiser;
	- Export to file (`LRC`);