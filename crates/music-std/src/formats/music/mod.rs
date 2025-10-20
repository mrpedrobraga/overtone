//! # Music
//!
//! A new format for interchange of structured things occurring over time.

use std::fmt::{Display, Formatter};
use serde::Serializer;

/// A "Music" file.
pub struct Music {
    meta: MusicMeta,
    content: Vec<MusicPattern>,
}

/// Metadata for a Music file.
pub struct MusicMeta {
    name: String,
    description: Option<String>,
    authors: Vec<String>,
}

pub struct MusicPattern {
    notes: Vec<Note>,
}

type NoteParameter = f32;

pub struct Note {
    pitch: f32,
    voice: usize,
}

pub enum Marker {
    SingleNoteMarker,
    MultiNoteMarker,
}

pub struct MarkerName {
    /// The namespace of the marker.
    /// Different plugins might provide markers with the same name,
    /// so the namespace helps with isolation.
    namespace: String,
    /// The name of the marker.
    ///
    /// The consumer of the music file will hopefully know what to do
    /// with a marker, given its name.
    name: String,
}

/// Marker names are ofter read as `namespace:name`.
/// So, for example, the Swing marker would be read `std:tempo_style_swing`.
impl Display for MarkerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_str(self.namespace.as_str())?;
        f.serialize_str(":")?;
        f.serialize_str(self.name.as_str())?;
        Ok(())
    }
}