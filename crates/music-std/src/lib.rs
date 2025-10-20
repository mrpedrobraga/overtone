use overtone::{overtone_plugin, plugin_prelude::*};

pub mod exporters;
pub mod formats;
pub mod renderers;

struct MusicStd;

impl Plugin for MusicStd {
    fn get_metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "music-std".to_string(),
            name: "Music Standard Library".to_string(),
            description: Some(
                "Default library containing lots of audio and musical functionality.".to_string(),
            ),
            authors: vec!["Overtone".to_string()],
        }
    }

    fn get_contributions(&self) -> PluginContributions {
        PluginContributions {
            renderers: Some(renderers::get()),
            exporters: Some(exporters::get()),
            contributions: vec![],
        }
    }
}

overtone_plugin! {
    Box::new(MusicStd)
}
