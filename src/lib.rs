//! # Overtone
//!
//! An API for management of musical projects,
//! that handles files, dependencies, plugins and actions,
//! while keeping the parts generic enough that you can
//! use them to do anything.
//!
//! ## Project
//!
//! An Overtone project is a folder which has an `Overtone.toml`
//! manifest.
//!
//! Here's a quick example of opening a project:
//! ```no_run
//! # use overtone::{ project::Project, errors::OvertoneApiError };
//! let mut p = Project::load_from_directory("./examples/test_project")?;
//! # Ok::<(), OvertoneApiError>(())
//! ```
//!
//! With that, you can modify the project in memory, until it's time to save it.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub mod api;
pub mod arrangement;
pub mod contributions;
pub mod plugin;
pub mod prelude;
pub mod project;
pub mod renderer;
pub mod resource;
pub mod serialization;
pub mod utils;
