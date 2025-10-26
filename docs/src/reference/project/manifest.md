# The `Overtone.toml` manifest

The file that makes a directory be recognized as an Overtone project.

Here's an example of the manifest of some project might look like.

```toml
[info]
name = "Funky Project"
authors = ["John Doe <doejohn@domain.com>"]

[dependencies]
some-plugin = { path = "./plugins/my_plugin.so" }
```

## The Schema

The manifest is typically a [TOML](https://toml.io/en/) file.

### Basic information

- `name : String`;
    - The name of your project;
- `description : String`;
    - A short description of your project;
- `authors : String[]`;
    - The names of the authors of your project;

### Overrides

As you've seen in [the project reference](..), Overtone stores its contents in
specific subdirectories. You can override what these are in this section.

- `compositions_dir : Path`;
    - the name of the directory where compositions are stored (defaults to `'compositions'`);
- `exports_dir : Path`;
    - the name of the directory where exports are placed (defaults to `'exports'`);

### Dependencies
- `dependencies : DependencyEntry[]`;
    - A section containing each dependency of your project. Each entry looks like this:
    - `path : Path`;
        - The path of your plugin on your file system;

> [!NOTE] Note to self
> It sure would be dandy if I could auto-generate these pages...