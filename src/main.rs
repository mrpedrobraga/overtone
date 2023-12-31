use overtone::project::Project;

fn main() {
    let p = Project::load_from_directory("./examples/test_project");

    let _ = dbg!(p);
}
