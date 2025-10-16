use {
    overtone::renderer::{ExportError, RenderExporter, RenderResult, RenderResultExt as _},
    std::collections::HashMap,
};
use crate::formats::pcm::{AudioPcm, PCM_RENDER_FORMAT_ID};

pub fn get() -> HashMap<String, Box<dyn RenderExporter>> {
    let mut map: HashMap<String, Box<dyn RenderExporter>> = HashMap::new();

    map.insert(
        "pcm-wav-exporter".to_string(),
        Box::new(PCMWavExporter {}) as Box<dyn RenderExporter>,
    );

    map
}

/// Renderer that emits audio from an arrangement.
#[derive(Default)]
pub struct PCMWavExporter {}

impl RenderExporter for PCMWavExporter {
    fn is_render_format_supported(&self, format_id: String) -> bool {
        format_id == PCM_RENDER_FORMAT_ID
    }

    fn export(
        &self,
        what: &dyn RenderResult,
        location: std::path::PathBuf,
    ) -> Result<(), ExportError> {
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
