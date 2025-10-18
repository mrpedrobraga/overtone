use overtone::plugin::PluginMetadata;
use overtone::{
    overtone_plugin,
    plugin::{Plugin, PluginContributions},
};

pub mod exporters;
pub mod formats;
pub mod renderers;

struct MusicStd {}

impl MusicStd {
    fn new() -> Self {
        MusicStd {}
    }
}

static MUSIC_STD_METADATA: std::sync::OnceLock<PluginMetadata> = std::sync::OnceLock::new();

impl Plugin for MusicStd {
    fn get_metadata(&self) -> &PluginMetadata {
        MUSIC_STD_METADATA.get_or_init(|| PluginMetadata {
            id: "music-std".to_string(),
            name: "Music Standard Library".to_string(),
            description: Some(
                "Default library containing lots of audio and musical functionality.".to_string(),
            ),
            authors: vec!["Overtone".to_string()],
        })
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
    Box::new(MusicStd::new())
}
