//! # SoundFont
//!
//! This module contains definition for a modern "sound font" format called "Timbres."

use serde::{Deserialize, Serialize};

/// A Pack, which contains several instruments.
#[derive(Serialize, Deserialize, Clone)]
pub struct Pack {
    pub schema_version: (u8,u8,u8),
    pub meta: PackMetadata,
    pub instruments: Vec<Instrument>
}

/// Metadata for a pack.
#[derive(Serialize, Deserialize, Clone)]
pub struct PackMetadata {
    /// The name of the pack.
    pub name: String,
    /// The version of the pack.
    pub version: String,
    /// The description of this pack, so you know what it's all about.
    pub description: Option<String>,
    /// The authors of this pack.
    /// Recommended to use the format "John Doe <johndoe123@email.com>"
    /// So that people know how to contact or credit the authors.
    pub authors: Vec<String>,
}

/// A single instrument, which is composed of several samples
/// a scheme for when and how to use each.
#[derive(Serialize, Deserialize, Clone)]
pub struct Instrument {
    pub meta: InstrumentMetadata,
    pub fragments: Vec<AudioFragment>,
    pub sampling_strategy: InstrumentSamplingStrategy
}

/// Metadata for an instrument.
#[derive(Serialize, Deserialize, Clone)]
pub struct InstrumentMetadata {
    /// The instrument's name.
    pub name: String,
    /// The instrument's description, for more juicy lore.
    pub description: Option<String>,
    /// The instrument's tags, which allow arbitrary categorizations in an editor,
    /// since an instrument can belong to multiple categories.
    ///
    /// For example, a drum kit might have categories: `Percussion`, `Acoustic`, `Recorded`.
    pub categories: Vec<String>,
}

/// A single audio fragment, which contains data
/// that can be played by a synth.
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum AudioFragment {
    // Raw pulse code modulation data.
    // It's simple, efficient and has perfect quality,
    // but might be overkill for some samples if they are _too big_.
    RawPCM(crate::formats::pcm::AudioPcm)
}

/// The method through which samples will be chosen.
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum InstrumentSamplingStrategy {
    /// Returns the reference sample closest to the chosen sample
    /// plus a delta for adjustment.
    EuclideanVoronoi(EuclideanVoronoiSamplingStrategy)
}

/// Reference samples will be placed across phase space.
/// When a point is requested from the set but isn't present,
/// this returns the closest reference sample as well as the "delta"
/// between the requested point and the reference sample.
///
/// For example, if you have two piano samples, one playing C3 and another playing C4
/// and you request F#3, the sampler will return a reference to the C3 sample and
/// a delta of six semitones. Then the synth uses the delta to pitch-shift the sample and
/// get a pretty good simulation of a "F#3" sample.
#[derive(Serialize, Deserialize, Clone)]
pub struct EuclideanVoronoiSamplingStrategy {
    /// The number of dimensions of this space.
    /// A dimension is "a way in which something can change,"
    /// so it represents a coordinated, ordered property of a sample.
    ///
    /// This sounds esoteric, so here's an example. A sample containing
    /// a piano recording has a specific pitch. Pitch is coordinated and ordered,
    /// because you can imagine an incremental variation you can have from low to high pitch.
    ///
    /// Other properties like this would be dynamics, staccato, vowel shape, etc.
    pub dimensionality: u8,
    /// The specific properties of each instrument _are_ arbitrary, so here we include
    /// the name of each dimension, so you don't waste time changing them in the editor
    /// wondering what the hell each number does.
    pub dimension_names: Vec<String>,
    /// The points representing the positions of the reference samples in phase space.
    ///
    /// # ATTENTION
    ///
    /// There is a correct way of iterating through this — in chunks of [`dimensionality`] items.
    ///
    /// ```rust
    /// # let points = vec![0, 1, 2, 0, 1, 2];
    /// # let dimensionality = 3;
    /// for point in points.chunks(dimensionality) {
    ///     dbg!(point);
    /// }
    /// ```
    pub points: Vec<u8>
}