//! A renderer is responsible for observing the arrangement and producing data for preview/exporting
//! as efficiently as possible.
//!
//! To actually preview or export rendered data, you'll need [`RenderPreviewer`] or [`render_exporter::RenderResultExporter`]

#![allow(dead_code)]

pub mod render_exporter;
pub mod render_result;
use self::render_result::RenderResult;
use crate::arrangement::Arrangement;

/// Trait for anything that can render an arrangement to a [`render_result::RenderResult`].
pub trait Renderer {
    /// Renders the given fragments using resources from the [`Arrangement`].
    /// This method is a task, so it should run asynchronously + notify progress.
    fn render(&self, arrangement: &Arrangement /* fragment slice */) -> Box<dyn RenderResult>;

    /// Returns an identifier used by previewers and exporters to trust that
    /// this renderer is providing data in the correct way -- that's because
    /// different plugins can not be strongly typed together since they're
    /// compiled in different environments.
    fn get_render_format_id(&self) -> String;
}
