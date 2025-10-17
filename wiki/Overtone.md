A music (and more) composition app with a powerful modular architecture.
## Project
A new project is where your multimedia adventure starts. A project is a directory
which contains an `Overtone.toml` file. Here's how the content of the manifest should look like.

```toml
[info]
name = "GuitarAndPianoProject"
license = "CC0"
authors = ["Pedro Braga"]

[editor]
requires_version = "0.0.1"

[plugins]
guitar-pro = { version = "3.0.0", path = "~/plugins/guitar-pro.so" }
fortepiano = { version = "2.0.1", path = "~/plugins/fortepiano2.0.1.so" }
```
## Editor
Editing a Project always happens through an Editor. When using an Editor, you can create new
actions (setting a field in a resource, calling a function) and then commit them to a project.

- Thorough Undo/Redo;
- Maintain invariants — it would be easy to break UUID links and other dependencies;
- Global Command Palette — you can invoke actions from anywhere using the Command Palette;
- Symmetry — different front-ends are equally powerful if they can perform the same actions;
- Multiplayer — serialising actions over a network for remote and/or collaborative editing;
## Rendering
But what is making music good for if you can't listen to your beautiful creation?

Every song has an active "Production Graph," which combines the fragments of your song and can generate several things.

![[concept_production_setup.png]]
