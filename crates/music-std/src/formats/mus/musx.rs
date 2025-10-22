//! # The MUSX (Musical Excerpt) format
//!
//! An excerpt of musical data that will be played
//! by a MUSi (Musical Instrument) or another MUS-compliant plugin.
//!
//! This format contains a bunch of notes in a sorted buffer,
//! such that they can be seeked efficiently using binary search.
//! Each note has
//!
//! It additionally contains two kinds of markers — time-bound and note-bound markers,
//! each which add additional notation to be interpreted by the MUSi.
//! These markers are also arbitrary. They may look something like this:
//!
//! `std::staccato`, `std::chord_stagger`, `std::artificial_harmonics`.

use serde::{Deserialize, Serialize};

type SortedVec<T> = Vec<T>;
type TimeOffset = f32;
type Map<K, V> = std::collections::BTreeMap<K, V>;
type Name = String;
type NoteIndex = usize;
type VoiceIndex = u32;
type KindIndex = u32;

/// An [`Excerpt`] with additional metadata so it can be stored on disk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub header: SongHeader,
    pub content: Excerpt,
}

/// A header that adds some context regarding how to interpret the data of an excerpt.
/// This has NO bearing on how the excerpt is actually interpreted or parsed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SongHeader {
    pub schema_version: u8,
    pub voice_names: Map<VoiceIndex, Name>,
    pub kind_names: Map<KindIndex, Name>,
}

/// An excerpt of music.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Excerpt {
    pub notes: SortedVec<Note>,
    pub timebound_markers: SortedVec<TimeboundMarker>,
    pub notebound_markers: SortedVec<NoteboundMarker>,
}

/// A note is a tiny range of time in which some sound is emitted by the instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    /// The start of the note as an offset from the beggining of the song.
    pub start: TimeOffset,
    /// The duration of the note.
    pub duration: TimeOffset,
    /// The "voice" this note belongs to. This isn't a parameter, because every excerpt should handle voices.
    pub voice: VoiceIndex,
    /// The intensity of the note, often called "velocity."
    pub intensity: f32,
    /// The pitch of the note, which is sometimes confusingly called "note."
    pub pitch: f32,
    /// The "kind" of the note, useful for instruments with many sound emitter kinds, like percussion,
    /// vocal instruments (think syllables), or perhaps even articulation.
    pub kind: KindIndex,
    /// A value used for intentionally choosing between samples when the three preceding parameters are all equal.
    pub sample_variant: u32,
    /// Another parameter you can use to fit some arbitrary data.
    pub extra_parameter: u32,
}

/// A Marker bound to the timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeboundMarker {
    pub start: TimeOffset,
    pub end: Option<TimeOffset>,
    pub content: Marker,
}

/// A Marker bound to a collection of notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteboundMarker {
    notes: Vec<NoteIndex>,
    pub content: Marker,
}

/// The name and parameters of a marker,
/// which are used by the MUSi to determine
/// what this marker means.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub name: Name,
    pub params: Map<Name, Value>,
}

/// The value of a marker's parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Numeric(f32),
    Text(Name),
}