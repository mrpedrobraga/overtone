//! # Contributions
//!
//! A plugin can offer several templates/bundles of functionality
//! that can be depended on by the arrangements and used by the host API.

use std::collections::BTreeMap;

pub struct PluginContributions {
    fragment_types: Option<PluginContributionEntry<i32>>,
}

pub type PluginContributionEntry<T> = BTreeMap<String, T>;
