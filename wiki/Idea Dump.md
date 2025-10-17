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
	- 