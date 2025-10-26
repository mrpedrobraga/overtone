//! # Editor API
//!
//! An API for editing complex projects consisting of interwoven resources
//! through serializable "Actions" that can be undone, reproduced, or sent over a network.
//!
//! The Editor API is agnostic regarding how each kind of resource is stored.
//! They might be stored in databases, on disk or in memory.

use arcstr::ArcStr;
use serde::{Deserialize, Serialize};
use editor::EditorClient;

pub mod resource;
pub mod project;
pub mod editor;

/// Implementations of these traits for the context of editing stuff on disk;
pub mod local;

pub type Name = ArcStr;