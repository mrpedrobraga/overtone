use overtone::renderer::RenderResult;

pub const PCM_RENDER_FORMAT_ID: &str = "audio-pcm";

/// A struct containing PCM audio.
/// Ideally, this would be a `RealTimeStream` so that it can be
/// previewed or exported in real time and concurrently.
pub struct AudioPcm {
    pub content: String,
}

impl RenderResult for AudioPcm {
    fn get_format_id(&self) -> String {
        PCM_RENDER_FORMAT_ID.to_owned()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
