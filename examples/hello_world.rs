use overtone::{errors::OvertoneApiError, project::Project};

fn main() -> Result<(), OvertoneApiError> {
    let mut _p = Project::load_from_directory("./examples/Untitled Project").unwrap();

    let plugin = _p.load_plugin("music-core".to_owned()).unwrap();

    let renderer = plugin.get_plugin().get_contributions().renderers.unwrap();
    let renderer = renderer.get("larynx").unwrap();

    dbg!(renderer.get_render_format());

    Ok(())
}
