use crate::formats::pcm::{AudioPcm, PCM_RENDER_FORMAT_ID};
use {
    overtone::{
        project::composition::Composition,
        renderer::{RenderResult, Renderer},
    },
    std::collections::HashMap,
};

pub fn get() -> HashMap<String, Box<dyn Renderer>> {
    let mut map: HashMap<String, Box<dyn Renderer>> = HashMap::new();

    map.insert(
        "audio-pcm-renderer".to_string(),
        Box::new(AudioPCMRenderer {}) as Box<dyn Renderer>,
    );

    map
}

/// Renderer that emits audio from an composition.
#[derive(Default)]
pub struct AudioPCMRenderer {}

impl Renderer for AudioPCMRenderer {
    fn render(&self, composition: &Composition /* fragment slice */) -> Box<dyn RenderResult> {
        let audio_pcm = AudioPcm::example();

        Box::new(audio_pcm)
    }

    fn get_render_format_id(&self) -> String {
        PCM_RENDER_FORMAT_ID.to_owned()
    }
}
