//! # Contributions
//!
//! A plugin can offer several templates/bundles of functionality
//! that can be depended on by the arrangements and used by the host API.

use crate::renderer::{render_exporter::RenderResultExporter, Renderer};
use std::collections::HashMap;
pub type PluginContributionBank<T> = HashMap<String, T>;
type OptBank<T> = Option<PluginContributionBank<T>>;

pub struct PluginContributions {
    pub renderers: OptBank<Box<dyn Renderer>>,
    pub exporters: OptBank<Box<dyn RenderResultExporter>>,
}
