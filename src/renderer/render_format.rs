/// Trait for a possible output from a renderer.
/// It'll be used by an exporter to preview + write files.
pub trait RenderFormat {
    fn get_name() -> String;
}
