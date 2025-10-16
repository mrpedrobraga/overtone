use std::path::PathBuf;

use overtone::{project::Project, OvertoneError};
use overtone::project::arrangement::{Arrangement};

fn main() -> Result<(), OvertoneError> {
    // Load project
    let mut _p = Project::load_from_directory("./examples/Untitled Project")?;

    // Load plugin in project named "music-core".
    let plugin_ref = _p.load_plugin("music-core".to_owned())?;

    // Get the renderer named "larynx"!
    let renderers = plugin_ref.get_plugin().get_contributions().renderers.unwrap();
    let renderer = renderers.get("larynx").unwrap();

    // Get the exporter named "larynx"!
    let exporters = plugin_ref.get_plugin().get_contributions().exporters.unwrap();
    let exporter = exporters.get("larynx").unwrap();

    // Load this song from disk (WIP).
    let song = Arrangement::load_from_directory(
        "./examples/Untitled Project/arrangements/Untitled Song".into(),
    )?;

    // Render the song to an intermediary render result.
    let render_result = renderer.render(&song);

    // Export the render result using the exporter to the file.
    let _ = dbg!(exporter.export(&*render_result, PathBuf::from("./examples/Untitled Project/exports/test_export.txt")));

    Ok(())
}
