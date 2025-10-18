use {
    overtone::{
        renderer::{ExportError, RenderExporter, RenderResult, RenderResultExt},
        transformer::{Node, SocketConnectionError, SocketIdx},
    },
    std::path::{Path, PathBuf},
};

fn main() {
    let n0 = WaveGenerator::new(440.0);
    let n1 = Gain::new(1.0);
    let n2 = WAVExporter::new("./examples/nodes/tune.wav");

    let sample = AudioPcm::example();

    let sample = n2.export(&sample);
}

struct WaveGenerator {
    frequency: f32,
}
impl WaveGenerator {
    pub fn new(frequency: f32) -> Self {
        WaveGenerator { frequency }
    }
}
impl Node for WaveGenerator {
    fn request_connect(
        &mut self,
        my_out_socket: SocketIdx,
        its_in_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        todo!()
    }
}

/// A struct containing PCM audio.
/// Ideally, this would be a `RealTimeStream` so that it can be
/// previewed or exported in real time and concurrently.
pub struct AudioPcm {
    pub sample_rate: usize,
    pub content: Vec<i16>,
}
impl RenderResult for AudioPcm {
    fn get_format_id(&self) -> String {
        "audio/pcm".to_string()
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
            let frequency = 440.0 * std::f32::consts::TAU;
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

struct Gain {
    gain: f32,
}
impl Gain {
    pub fn new(gain: f32) -> Self {
        Gain { gain }
    }
}

impl Node for Gain {
    fn request_connect(
        &mut self,
        my_out_socket: SocketIdx,
        its_in_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        todo!()
    }
}

struct WAVExporter {
    file: PathBuf,
    sample_rate: usize,
}
impl WAVExporter {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        WAVExporter {
            file: PathBuf::from(path.as_ref()),
            sample_rate: 41000,
        }
    }
}
impl Node for WAVExporter {
    fn request_connect(
        &mut self,
        my_out_socket: SocketIdx,
        its_in_socket: SocketIdx,
    ) -> Result<(), SocketConnectionError> {
        todo!()
    }
}
impl RenderExporter for WAVExporter {
    fn is_render_format_supported(&self, format_id: String) -> bool {
        format_id == "audio/pcm"
    }

    fn export(&self, what: &dyn RenderResult) -> Result<(), ExportError> {
        let location = &self.file;
        let audio_pcm = what
            .as_::<AudioPcm>()
            .ok_or(ExportError::IncorrectRenderFormat)?;
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: audio_pcm.sample_rate as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(location, spec).expect("Failed to write.");

        for sample in audio_pcm.content.iter().copied() {
            writer.write_sample(sample).unwrap();
        }

        writer
            .finalize()
            .expect("Error finalising to write the wav file.");

        Ok(())
    }
}
