# Overtone

A music (and more) composition app with a powerful modular architecture.

> 🚧 This project is in development and can not be used for... anything yet.

## Goals

- Cross-platform — run on PC, Mac, mobile, web, maybe even embedded;
- Lightweight — this program should be performant and have a low memory footprint, leaving enough RAM for your audio endeavours;
  - Lazy-loaded — while editing a song, only the resources in use are loaded, saving memory;
- Stable — using memory safe patterns, thorough error definitions, and concurrency for long-lived tasks, this program shouldn't be freezing or crashing on you;
  - I particularly care about good dynamic error messages and popups for the user;
- Modular — it's easy to create new renderers, exporters, previewers, etc. Overtone is like a "host" for the many plugins that interconnect;
  - Projects can have "dependencies" on plugins and external resources;
- Text-based — you can use VCS to keep track of changes in your compositions, as well as inspect the contents of your songs. This way, you truly OWN your project, you can interact with your song even without Overtone;

## Features

This library comes with a binary, too, a CLI tool for editing your project directly on disk.

You can:
- [x] `overtone new` to create a new project.
- [ ] `overtone package` to package the project into a portable format (all external resources it uses will be bundled);
- [x] `overtone render` to render a project to a file (accepts options);
- [ ] `overtone preview` to preview a project to a file (accepts options);

`music-std`, the builtin plugin, comes with its own features, too.

- Music concepts;
  - External audio samples; 
  - Piano roll for keyboard-like instruments;
  - Guitar roll for string instruments;
  - Arbitrary musical markings for the song and for notes (list below non-exhaustive);
    - Dynamics (pp, p, f, ff);
    - Ornaments (portamento, trill, acciacatura);
    - Articulations (pizzicato, harmonics, sul tasto);
    - Tempo markings (swing, time signatures, fermata);
    - Lyrics;
  - Non-destructive, graph-based effect composition (like Blender's geometry nodes and modifiers);
    - Audio effects for manipulating audio;
    - Note effects for manipulating/generating note data;
- Music playback;
  - [ ] `SFLT`, a SoundFont player that supports `SF2`, courtesy of [ash taylor](https://github.com/estroBiologist/);
  - [ ] `Timbre`, a player for overtone's own sample library;
  - [ ] `Vibrando`, a VST player;
  - [x] Unique `music-std` plugins;