use overtone::project::Project;

fn main() {
    let p = Project::load_from_directory("./examples/test_project");

    let p = match p {
        Ok(v) => v,
        Err(e) => {
            dbg!(e);
            return;
        }
    };

    let _ = println!("{:#?}", p);
}
