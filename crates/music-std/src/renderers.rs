use {
    crate::formats::{AudioPcm, PCM_RENDER_FORMAT_ID},
    overtone::{
        project::arrangement::Arrangement,
        renderer::{RenderResult, Renderer},
    },
    std::collections::HashMap,
};

pub fn get() -> HashMap<String, Box<dyn Renderer>> {
    let mut map: HashMap<String, Box<dyn Renderer>> = HashMap::new();

    map.insert(
        "audio-pcm-renderer".to_string(),
        Box::new(AudioRenderer {}) as Box<dyn Renderer>,
    );

    map
}

/// Renderer that emits audio from an arrangement.
#[derive(Default)]
pub struct AudioRenderer {}

impl Renderer for AudioRenderer {
    fn render(&self, arrangement: &Arrangement /* fragment slice */) -> Box<dyn RenderResult> {
        Box::new(AudioPcm {
            content: arrangement.meta.name.clone(),
        })
    }

    fn get_render_format_id(&self) -> String {
        PCM_RENDER_FORMAT_ID.to_owned()
    }
}
