use super::render_result::RenderResult;
use std::path::PathBuf;

pub enum ExportError {
    /// The parameter passed to `what` which hides a value through the [`RenderResult`] trait
    /// has an unsupported type (yes, despite the `is_render_format_supported` check).
    IncorrectRenderFormat,
}

pub trait RenderResultExporter {
    /// Returns true if this tag corresponds to a format you are optimistic about being able to export.
    /// The formats that can be here are not defined by overtone, so try to pick a meaningful name and checking the community.
    /// This, btw, should be the format that goes into the exporter, not out of it.
    ///
    /// For example, exporting audio to ogg should be something like `pcm_audio`.
    /// If exporting video could be something like `rgba_frames` or `tmp_png_frames_dir`.
    fn is_render_format_supported(format_id: String) -> bool;

    /// Exports a render result to a location.
    fn export(what: &dyn RenderResult, location: PathBuf) -> Result<(), ExportError>;
}
