use overtone::transformer::Value;

/// A struct containing PCM audio.
/// Ideally, this would be a `RealTimeStream` so that it can be
/// previewed or exported in real time and concurrently.
pub struct AudioPcm {
    pub sample_rate: usize,
    pub content: Vec<f32>,
}

impl Value for AudioPcm {
    fn get_format_name(&self) -> String {
        "audio/pcm".to_string()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl AudioPcm {
    pub fn example(frequency: f32) -> Self {
        let sample_rate = 41000;
        let mut content = Vec::with_capacity(sample_rate);

        for i in 0..sample_rate {
            let t = i as f32 / sample_rate as f32;
            let frequency = frequency * std::f32::consts::TAU;
            let amplitude = (1.0f32 - 0.985f32).powf(t);
            let sample = (t * frequency).sin();
            let sample = sample.signum();
            let sample = sample * amplitude;
            content.push(sample);
        }

        Self {
            sample_rate,
            content,
        }
    }
}