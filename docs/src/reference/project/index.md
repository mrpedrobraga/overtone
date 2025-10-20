# Projects

An Overtone project is any and every directory which contains a valid `Overtone.toml` manifest at its root.

```
MyProject/
    - Overtone.toml
```

Inside of a Project there will be a bunch of stuff. If you created a project through Overtone, it most like looks like this:

```
MyProject/
    arrangements/
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
name = "Funky Project"
authors = ["John Doe <doejohn@domain.com>"]

[plugins]
some-plugin = { "local_path": "./plugins/my_plugin.so" }
```

The full Manifest specification is available [here](./manifest.md);