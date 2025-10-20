# Getting Started (For Developers)

Overtone and its builtin plugins are mostly Rust source code, with a bit of C here and there.
The source code is available at [its GitHub repository](https://github.com/mrpedrobraga/overtone), with an MIT license.

Rust is the officially supported language for developing Overtone plugins.
You can [install it](https://rust-lang.org/tools/install/) from here, it doesn't take long.
You can add `overtone` as a dependency in a `cargo` project through
[crates.io](https://crates.io/crates/overtone) or by adding:
```toml
overtone = { git = "https://github.com/mrpedrobraga/overtone.git" }
```
to your `Cargo.toml`.
