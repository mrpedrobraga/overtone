//! # Rendering
//!
//! A [`Renderer`] is an object that observes an [`Composition`] and produces an intermediary
//! object (`dyn RenderResult`) that can be either previewed or exported in different ways.
//!
//! ## Previewing
//!
//! Not available yet.
//!
//! ## Exporting
//!
//! [`RenderExporter`] is the trait for exporters, which you can have several of
//! for the same format of render result.
//!
//! The reason why it's a trait is if a song is rendered to, say, audio in PCM,
//! it can be encoded into WAV, OGG, MP3, etc.

#![allow(dead_code)]

use crate::project::composition::Composition;
use std::path::PathBuf;

/// Trait for anything that can render an composition to a [`RenderResult`].
#[deprecated]
pub trait Renderer {
    /// Renders the given elements using resources from the [`Composition`].
    ///
    /// TODO: Make this method an asynchronously running task that can be probed and cancelled.
    fn render(&self, composition: &Composition /* fragment slice */) -> Box<dyn RenderResult>;

    /// Returns an identifier used by previewers and exporters to identify
    /// the type hidden behind the opaque `dyn RenderResult`.
    fn get_render_format_id(&self) -> String;
}

/// Trait for a possible output from a renderer.
/// It'll be used by an exporter to preview + write files.
pub trait RenderResult {
    /// Returns the identifier of this render format, which
    /// will serve as a dynamic check of whether two endpoints which
    /// require a [`RenderResult`] can connect.
    ///
    /// This is still just a light check for convenience, and retrieving
    /// a value from this still produces an Option of the underlying type.
    fn get_format_id(&self) -> String;

    /// Returns this render result as an [`std::any::Any`];
    ///
    /// This isn't a given method because you're not converting the
    /// `dyn RenderResult` you are probably going to call this method in
    /// to an any — you're converting the original concrete type to `Any`.
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait RenderResultExt: RenderResult {
    /// Returns this render result as the concrete type `T`, if possible.
    ///
    /// This is used by exporters and previewers to get some concrete data
    /// out of the render result.
    fn as_<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl<T> RenderResultExt for T where T: RenderResult + ?Sized {}

/// Trait for something that can export a render result to a file.
pub trait RenderExporter {
    /// Returns true if this tag corresponds to a format you are optimistic about being able to export.
    /// The formats that can be here are not defined by overtone, so try to pick a meaningful name and checking the community.
    /// This, btw, should be the format that goes into the exporter, not out of it.
    ///
    /// For example, exporting audio to ogg should be something like `pcm_audio`.
    /// If exporting video could be something like `rgba_frames` or `tmp_png_frames_dir`.
    fn is_render_format_supported(&self, format_id: String) -> bool;

    /// Exports a render result to a location.
    fn export(&self) -> Result<(), ExportError>;
}

#[derive(Debug)]
pub enum ExportError {
    /// The format hidden behind the opaque `dyn RenderResult` is not supported by this exporter.
    /// If you got this error, it means someone failed to implement
    /// proper validation in `is_render_format_supported.`
    IncorrectRenderFormat,
    /// Error saving the file to disk.
    IOError(std::io::Error),
    /// No location chosen
    NoTargetLocationChosen,
    /// Too lazy to implement a proper error right now during this
    /// stage of iterative development. You know how it is.
    ///
    /// This WILL be deleted later.
    #[deprecated]
    Generic
}
