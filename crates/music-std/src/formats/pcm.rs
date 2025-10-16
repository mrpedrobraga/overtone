use serde::{Deserialize, Serialize};
use overtone::renderer::RenderResult;

pub const PCM_RENDER_FORMAT_ID: &str = "audio-pcm";

/// A struct containing PCM audio.
/// Ideally, this would be a `RealTimeStream` so that it can be
/// previewed or exported in real time and concurrently.
#[derive(Serialize, Deserialize, Clone)]
pub struct AudioPcm {
    pub sample_rate: usize,
    pub content: Vec<i16>,
}

impl RenderResult for AudioPcm {
    fn get_format_id(&self) -> String {
        PCM_RENDER_FORMAT_ID.to_owned()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl AudioPcm {
    pub fn example() -> Self {
        let sample_rate =  44100;
        let mut content = Vec::new();

        for t in (0..sample_rate).map(|x| x as f32 / sample_rate as f32) {
            let sample = (t * (440.0 - t * 220.0) * 2.0 * std::f32::consts::PI).sin();
            let amplitude = i16::MAX as f32;
            content.push((sample * amplitude) as i16);
        }

        Self { sample_rate, content }
    }
}