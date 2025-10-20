---
outline: deep
---

# Your First Plugin

> [!NOTE]
> This guide assumes you either are mildly proficient with Rust, or are very willingly to learn it
> along the way. We recommend [The Rust Book](https://doc.rust-lang.org/book/) to use as reference
> while developing for the first time.
>
> As a reminder, you can install the Rust toolchain [from here](https://rust-lang.org/tools/install/);

## 0. Create a new Rust library using the `cargo` package manager, which you hopefully installed;

```sh
cargo init --lib <name>
```

> This documentation will be assuming `yourplugin` as the chosen name of your library.

## 1. Set up your `Cargo.toml`;

Add `overtone` as a dependency.

```toml
[dependencies]
overtone = { git = "https://github.com/mrpedrobraga/overtone.git" }
```

Make sure the library is compiling as a dynamic library.
This is important, because it makes sure the plugin compiles to a file that can be shared around.

```toml
[lib]
crate-type = ["dylib", "rlib"]
```

You can find the `Cargo.toml` documentation [here](https://doc.rust-lang.org/cargo/reference/index.html).

## 2. Implement `Plugin`;

The crux of a plugin library is a function that returns some struct that implements the `Plugin` trait;

```rust
use overtone::{overtone_plugin, plugin_prelude::*};

struct MyPlugin;

impl Plugin for MyPlugin {
    fn get_metadata(&self) -> &PluginMetadata {
        /* Metadata */
    }

    fn get_contributions(&self) -> PluginContributions {
        /* Contributions */
    }
}

/// This macro creates an entry point in the library
/// that `overtone` will use to ask for a plugin.
overtone_plugin! {
    Box::new(MyPlugin)
}
```

Let's look at it in parts.

### 2.1 Metadata

The plugin's metadata contains some information about the plugin that will be displayed in the editor.
For example, here's the metadata from the `music-std` builtin plugin.

```rust
fn get_metadata(&self) -> &PluginMetadata {
    PluginMetadata {
        id: "music-std".to_string(),
        name: "Music Standard Library".to_string(),
        description: Some(
            "Default library containing lots of audio and musical functionality.".to_string(),
        ),
        authors: vec!["Overtone".to_string()],
    }
}
```

- `id` is the identifier of your plugin, and it must be unique. It's used to namespace the contributions your plugin provides;
- `name` is a `String` with the display name of your plugin;
- `description` is a `String` with the description of your plugin. It's okay if it's brief, since every contribution gets its own description;
- `authors` is a `Vec<String>` and contains all the authors that created this plugin;

### 2.2 Contributions

Contributions are the things your plugin adds to Overtone.
We'll see more about them in the next chapter.

For now, provide an empty contribution set so that the library compiles.

```rust
fn get_contributions(&self) -> PluginContributions {
    PluginContributions::empty()
}
```

## 3. Let's see what we've got...

Check that it all compiles using:

```sh
cargo build
```

If it does, you'll find the dynamic library in `target/debug/`,
as a file named `libyourplugin.so` (if you're on Linux)
or `libyourplugin.dll` (if you're on Windows);

To build in release mode, which enables speed and size optimisations, use.

```sh
cargo build --release
```

And the library will be found in `target/release/`.

This is the file that `Overtone` will load whenever you want to use the plugin.