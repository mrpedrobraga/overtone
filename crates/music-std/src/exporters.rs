use {
    crate::formats::{AudioPcm, PCM_RENDER_FORMAT_ID},
    overtone::renderer::{ExportError, RenderExporter, RenderResult, RenderResultExt as _},
    std::{collections::HashMap, io::Write},
};

pub fn get() -> HashMap<String, Box<dyn RenderExporter>> {
    let mut map: HashMap<String, Box<dyn RenderExporter>> = HashMap::new();

    map.insert(
        "pcm-wav-exporter".to_string(),
        Box::new(AudioExporter {}) as Box<dyn RenderExporter>,
    );

    map
}

/// Renderer that emits audio from an arrangement.
#[derive(Default)]
pub struct AudioExporter {}

impl RenderExporter for AudioExporter {
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
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(location)
            .map_err(ExportError::IOError)?;
        file.write_all(audio_pcm.content.as_bytes())
            .map_err(ExportError::IOError)?;
        Ok(())
    }
}
