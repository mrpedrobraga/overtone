use overtone::plugin::PluginDependencyEntry;
use overtone::project::{Project, ProjectInfo, ProjectManifest};
use std::path::PathBuf;

fn main() {
    let project = Project::new(ProjectManifest {
        info: ProjectInfo {
            name: "Funky Project".to_string(),
            authors: vec!["Pedro Braga <mrpedrobraga.com>".to_owned()],
        },
        configuration_overrides: Default::default(),
        plugins: vec![PluginDependencyEntry {
            id: "music-std".to_string(),
            path: PathBuf::from("../../../target/release/libovertone_music_std.so"),
        }],
    });

    project.save_to_new_directory("./examples/new_project/").unwrap();
}
