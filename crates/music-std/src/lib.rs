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

impl Plugin for MusicStd {
    fn get_id(&self) -> &'static str {
        "music-std"
    }

    fn get_contributions(&self) -> PluginContributions {
        PluginContributions {
            renderers: Some(renderers::get()),
            exporters: Some(exporters::get()),
        }
    }
}

overtone_plugin! {
    Box::new(MusicStd::new())
}
