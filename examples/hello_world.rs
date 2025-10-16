use std::path::PathBuf;

use overtone::project::arrangement::Arrangement;
use overtone::{project::Project, OvertoneError};

fn main() -> Result<(), OvertoneError> {
    let mut _p = Project::load_from_directory("./examples/Untitled Project")?;

    let plugin_ref = _p.load_plugin("music-std".to_owned())?;

    let renderers = plugin_ref
        .get_plugin()
        .get_contributions()
        .renderers
        .expect("Plugin does not have renderer contributions.");
    let renderer = renderers
        .get("audio-pcm-renderer")
        .expect("Renderer not found.");

    let exporters = plugin_ref
        .get_plugin()
        .get_contributions()
        .exporters
        .expect("Plugin does not have exporter contributions.");
    let exporter = exporters
        .get("pcm-wav-exporter")
        .expect("Exporter not found");

    let song = Arrangement::load_from_directory(
        "./examples/Untitled Project/arrangements/Untitled Song".into(),
    )?;

    let render_result = renderer.render(&song);

    let _ = dbg!(exporter.export(
        &*render_result,
        PathBuf::from("./examples/Untitled Project/exports/test-export.wav")
    ));

    Ok(())
}
