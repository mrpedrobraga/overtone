//! # Contributions
//!
//! A plugin can offer several templates/bundles of functionality
//! that can be depended on by the arrangements and used by the host API.

use std::collections::HashMap;
use crate::renderer::Renderer;
pub type PluginContributionBank<T> = HashMap<String, T>;
type OPE<T> = Option<PluginContributionBank<T>>;

pub struct PluginContributions {
    pub renderers: OPE<Box<dyn Renderer>>,
}