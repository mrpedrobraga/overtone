use overtone::{errors::OvertoneApiError, project::Project};

fn main() -> Result<(), OvertoneApiError> {
    let mut p = Project::load_from_directory("./examples/Untitled Project")?;

    let pl = p.load_plugin("music-core".to_string());

    let _ = dbg!(pl);

    Ok(())
}
