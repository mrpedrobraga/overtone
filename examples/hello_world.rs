use overtone::{
    arrangement::{serialization::load_arrangement_from_directory, Arrangement},
    errors::OvertoneApiError,
    project::Project,
};

fn main() -> Result<(), OvertoneApiError> {
    let mut _p = Project::load_from_directory("./examples/Untitled Project").unwrap();

    let plugin = _p.load_plugin("music-core".to_owned()).unwrap();

    let renderers = plugin.get_plugin().get_contributions().renderers.unwrap();
    let renderer = renderers.get("larynx").unwrap();

    let exporters = plugin.get_plugin().get_contributions().exporters.unwrap();
    let exporter = exporters.get("larynx").unwrap();

    let ref song = Arrangement {
        header: load_arrangement_from_directory(
            "./examples/Untitled Project/arrangements/Untitled Song".into(),
        )
        .map_err(|e| OvertoneApiError::ArrangementError(e))?,
    };

    let render_result = renderer.render(song);
    dbg!(render_result.get_format_id());
    let _ = dbg!(exporter.export(&*render_result, "./examples/test_export.txt".into()));

    Ok(())
}
