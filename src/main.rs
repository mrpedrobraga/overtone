use overtone::project::Project;

fn main() {
    let p = Project::load_from_directory("./examples/test_project");

    let mut p = match p {
        Ok(v) => v,
        Err(e) => {
            dbg!(e);
            return;
        }
    };

    let ids = match &p.file.plugins {
        None => None,
        Some(v) => Some(v.iter().map(|pde| pde.id.clone()).collect::<Vec<_>>()),
    };

    let pl = match &ids {
        None => return,
        Some(v) => p.load_plugin(v[0].clone()),
    };

    let _ = dbg!(pl);
}
