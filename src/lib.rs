//! # Overtone
//!
//! An api for management of musical-ish projects,
//! that handles files, dependencies, plugins and actions,
//! while keeping the parts generic enough that you can
//! use them to do anything.

pub mod errors;
pub mod info;
pub mod plugin;
pub mod project;
pub mod serialization;
pub mod types;
mod utils;
