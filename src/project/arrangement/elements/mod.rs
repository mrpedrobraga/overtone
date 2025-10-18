//! # Fragments
//!
//! ...are the building blocks of Arrangements. By combining elements, you can make songs!

use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::project::arrangement::time::Moment;

/// The trait that represents something that can be composed in a song.
pub trait Element {
    fn get_id(&self) -> String;
}

/// Cool metadata for a track.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementDecoration {
    label: String,
    color: Color
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32
}

// -- Multi tracks -- //

/// The linear multi-track fragment is like a big drawer with multiple shelves
/// where sub-elements can be placed, taking some space.
///
/// When rendering, as the time cursor goes from beginning to end, the track
/// recursively asks its sub-elements to render, taking their [`TrackItemTransform`] into account.
///
/// Here's a cool little diagram.
///
/// ```no-run
/// 0: [          [AAAAA]  [BBBBBBBBB]        ]
/// 1: [      [CCCCC]     [AAAAAAAAAAAAAA]    ]
/// ```
pub struct LinearMultiTrackElement {
    items: Vec<TrackItemElement>,
    decoration: ElementDecoration
}

/// An item inside a track.
pub struct TrackItemElement {
    transform: TrackItemTransform,
    decoration: ElementDecoration,
    content: Box<dyn Element>
}

/// The transform of a single track item.
#[derive(Debug, Clone)]
pub struct TrackItemTransform {
    /// The track the item is on.
    track: u8,
    /// The position of the start of this embedded track.
    position: Moment,
    /// If present, this item will consist of merely a slice
    /// of whatever data it holds.
    ///
    /// The tuple here represents `(start_moment, end_moment)`,
    /// and allow you to single out a specific part of a larger fragment.
    slice: Option<(Moment, Moment)>,
    /// The time stretch of this track.
    scale: f32,
}

impl Element for LinearMultiTrackElement {
    fn get_id(&self) -> String {
        "multi-track".to_string()
    }
}

// -- File -- //

/// Fragment that contains a reference to a file resource.
///
/// You can use this to sample audio files, for example.
pub struct FileElement {
    reference: Box<Path>
}

// -- Text -- //

/// Nice fragment that contains a comment. For commenting on things, you know.
pub struct CommentElement {
    text: String
}
