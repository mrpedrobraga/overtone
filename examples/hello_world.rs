use overtone::{errors::OvertoneApiError, project::Project};

fn main() -> Result<(), OvertoneApiError> {
    let p = Project::load_from_directory("./examples/Untitled Project").unwrap();

    Ok(())
}
