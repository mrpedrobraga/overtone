use serde::{Deserialize, Serialize};
use {core::f32, overtone::renderer::RenderResult};

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
        let sample_rate = 41000;
        let mut content = Vec::with_capacity(sample_rate);

        for i in 0..sample_rate {
            let t = i as f32 / sample_rate as f32;
            let frequency = 440.0 * f32::consts::TAU;
            let amplitude = (1.0f32 - 0.985f32).powf(t);
            let sample = (t * frequency).sin();
            let sample = sample.signum();
            let sample = sample * amplitude;
            let sample16: i16 = (sample * (i16::MAX as f32)) as i16;
            content.push(sample16);
        }

        Self {
            sample_rate,
            content,
        }
    }
}
