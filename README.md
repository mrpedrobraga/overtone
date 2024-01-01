# Overtone

An API for musical-ish projects, that abstracts loading, managing and editing
projects, arrangements, assets and plugins that can be combined to make music.

> [!ALERT] 🚧 This is an in-progress project, in a rather early stage.
> It's not ready for serious usage yet.

It reinvents the wheel somewhat in music creation, having focus on *Rich Musical Components* that better convey musical ideas for musicians reading and writing music in Overtone. Instead of forcing MIDI to do what it was never designed to do (express and render realistic music), you can decorate your music with markings (Glissandi, Hammer-ons, Pizz., Sforzando, etc.) that better model how a human perceives music, while still having meaningful playback from Overtone audio plugins.

Furthermore, none of the musical features are baked in, so musicians of all cultures and walks can join together to work on a shared space.

## Project

An Overtone project is simply a folder with an `Overtone.toml` file inside.

```toml
[info]
name = "Untitled Project"
license = "MIT"
authors = ["Pedro Braga"]

[editor]
requires_version = "0.0.1"

[[plugins]]
id = "music-core"
path = "./plugins/music-core.so"
```

With that, you can load the Project like this:
```rust
Project::load_from_directory("./examples/Untitled Project")?;
```

The rest of the API will allow you to create new arrangements, manage arrangements and external dependencies, and call the renderers in a safe manner.

## Design Philosphy

### Free as in love <3

Overtone is free open source software, and will always be. It doesn't have any cloud-based functionality, nor subscriptions. Pay nothing, own it forever.

### Minimal vendor lock-in

Your project isn't an obscured file, but a plain directory. What can't be saved as plain TOML (because of their size) will be saved in an open binary format: **STIF** (STructure Interchange Format).

### Modular

Overtone doesn't provide a lot of specific music features, to remain lightweight. Instead, features, even basic ones, are provided through `plugins`. If an Overtone front-end is implemented, it'll probably come with a few core plugins bundled, but the idea is that: you pick what you need, and you can swap even basic functionality with other plugins. This mirrors the scenario with programming languages and package managers (I have `cargo` in mind).

Overtone will manage your plugin dependencies and let you know what plugins are missing if you intend to open a file, and which version is needed, too.

Overtone is lightweight and fast. It achieves this by sticking to one goal, and avoiding bloat. So far, a sign of the project's maturity is that as it evolves, it gets smaller and smaller.

### Safe

Overtone is implemented in Rust, but more than that, it has care for its internal failure cases, always returning useful and rich errors so that the program doesn't crash nor freeze.

Every function that can fail should return an instance of the enum `OvertoneApiError` which delightfully describes the failures.

An Overtone front-end can avoid crashes by enjoying the internal dependency model, where some areas of Overtone are blocked if their dependencies are missing or have a problem (A `TrackFragment` can't play a sound if its `.mp3` file is missing; Overtone will not crash, instead, the front-end can display a helpful icon and a popup).

### Helpful

Overtone aims to have the best UX, always providing useful context in its data types that can prevent common mistakes. For example, Overtone keeps tracks of the version of the project, arrangements, and plugins that were used, so that you don't accidentally corrupt your files.

### Fresh

Good ol' file formats (such as `.mid`) are always reliable, and it seems like there's nothing to gain from reinventing formats (and a lot of time to lose).

While standard formats will be wonderfully supported in Overtone, it'll use its own formats (which are always supersets) as compatibility layer. Richer formats can be more efficient, and better aligned with the interests of today's musicians. `mid`, `sf2`, `vst` all did their best, but one thing we have that they don't is the incredible **benefit of hindsight**.

That hindsight informs Overtone to be **simpler**, and offer functionality through elegant design and composability, rather than stacking feature on top of feature.


## Abstraction

This API abstracts over musical concepts, allowing the community to develop
features of any kind.

An `overtone` plugin might contribute with any of the following:

### Renderer
A program that can render your arrangement into one or many exportable formats. The default audio renderer falls under this category.

> Imagine implementing exporters for .ogg, .vst, .mscx, .png (scores), .mp4;

### Instrument
A program that's associated to track fragments and declares how that fragment should be interpreted when rendering. `SF2Player`, `Heisenberg (and waveform synths)`, `VST` fall under this category.

> Music instruments such as Guitar, Piano, Square Wave, SF2Player,
> but can be used for projects that don't render audio, too.

### Transformer
A program that transforms data as it comes from an instrument, before it is rendered. `VST` effects fall under this category.

> Think audio (or visual, or whatever) effects.

### Track Fragment Type
A new type of Track Fragment -- building blocks of your arrangement spread across tracks in time.
The frontend can implement different UI for each type of track fragment. When rendered, each fragment contributes to the final piece.

> Piano Roll Pattern, Drum Pattern, Harp Pattern, Violin Pattern,
> Images, Audio Samples, Comments and Notes, etc.

### Resource Type
A new type of resource (asset) that can be managed and read from overtone.
Imported assets from `.ogg`, `.mp3`, `.png` fall under such category.

## Contributing

Contributing is Welcome, although the project is now in a very early stage.

Monetary contributions aren't accepted yet -- but will be welcomed in the future.

## License

***In Progress***

~~I'm aiming for MIT, but I need to check the deps carefully first.~~