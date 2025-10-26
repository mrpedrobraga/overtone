# Projects

An Overtone project is any and every directory which contains a valid `Overtone.toml` manifest at its root.

```
MyProject/
    - Overtone.toml
```

Inside a Project there will be a bunch of stuff. If you created a project through Overtone, it most like looks like this:

```
MyProject/
    compositions/
    plugins/
    exports/
    Overtone.toml
```

Let's see what goes inside a project.

## The Manifest

The file that marks a folder as a project. It also contains important basic information regarding the project's structure and dependencies.
It is a file that looks somewhat like this:

```toml
[info]
name = "GuitarAndPianoProject"
license = "CC0"
authors = ["Pedro Braga"]

[editor]
requires_version = "0.0.1"

[dependencies]
guitar-pro = { version = "3.0.0", path = "/plugins/guitar-pro.so" }
fortepiano = { version = "2.0.1", path = "/plugins/fortepiano2.0.1.so" }
```

The full Manifest specification is available [here](./manifest);