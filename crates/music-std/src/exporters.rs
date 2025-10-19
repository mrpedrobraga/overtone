use crate::formats::pcm::{AudioPcm, PCM_RENDER_FORMAT_ID};
use {
    overtone::renderer::{ExportError, RenderExporter, RenderResult, RenderResultExt as _},
    std::{collections::HashMap, path::PathBuf},
};

pub fn get() -> HashMap<String, Box<dyn RenderExporter>> {
    let mut map: HashMap<String, Box<dyn RenderExporter>> = HashMap::new();

    map
}
