//! # Overtone
//!
//! An API for management of musical-ish projects,
//! that handles files, dependencies, plugins and actions,
//! while keeping the parts generic enough that you can
//! use them to do anything.
//!
//! An Overtone project is a folder which has an `Overtone.toml`
//! file in its root.
//!
//! As such, here's a quick example of opening a project:
//! ```no_run
//! # use overtone::{ project::Project, errors::OvertoneApiError };
//! let mut p = Project::load_from_directory("./examples/test_project")?;
//! # Ok::<(), OvertoneApiError>(())
//! ```
//!
//! With that, you can modify the project in memory, until it's time to save it.

pub mod errors;
pub mod info;
pub mod plugin;
pub mod project;
pub mod serialization;
pub mod types;
mod utils;
