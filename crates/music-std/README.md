# Overtone Music Standard Library (`music-std`)

This library offers an array of useful plugins for making music with Overtone.

It might be surprising that `overtone` itself doesn't come with audio or music
capabilities. Instead, those basic features are provided by this plugin,
which comes bundled in with the parent crate anyways. This is effectively
a promise that Overtone plugins will be powerful and flexible enough that
any plugin by party can feel "built-in".

## Features

All the contributions this plugin offers.

### Signal Formats
- [ ] Audio streams (PCM);
- [ ] Music;
- [ ] MIDI;

### File Formats
- [ ] MUS (Overtone Music)
- [ ] MIDI (Musical Instrument Digital Interface)
- [ ] SFT (SoundFont for Timbres)
- [ ] SF2 (SoundFont 2.0)

### Nodes
#### Transformers
- [ ] Timbres, an SFT sampler;
- [ ] [SFLT](https://github.com/estroBiologist), a SF2 sampler;
- [ ] [LILV](https://drobilla.net/docs/lilv/), a LV2 host;
- [ ] VSTi (im workshopping this one yet);
- Effects
    - [ ] Standard Gain;
    - [ ] Standard Compressor;
    - [ ] Standard Parametric EQ;
    - [ ] Standard Delay;

#### Sources
- [ ] Real time audio sources, using [CPAL](https://crates.io/crates/cpal/);
- [ ] Audio Sampler;
    - Supporting WAV, MP3, OGG, QOA;

#### Sinks
- [ ] WAV Exporter, using [hound](https://crates.io/crates/hound);
- [ ] LRC Exporter;
- [ ] ID3 Metadata, likely using [id3](https://crates.io/crates/id3);
- [ ] Real time audio sinks, using [CPAL](https://crates.io/crates/cpal/);

### Editors (through UI Composer)
- [ ] Music Editor (for MUS);
- [ ] MIDI Editor (for MIDI);
- [ ] Audio Viewer;