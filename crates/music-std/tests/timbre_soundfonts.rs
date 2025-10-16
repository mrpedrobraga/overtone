use std::io::Write;
use overtone_music_std::formats::timbre::*;

#[test]
fn store() {
    let sample_1 = overtone_music_std::examples::get_example_pcm_sample();

    let soundfont = Pack {
        schema_version: (0, 0, 0),
        meta: PackMetadata {
            name: "Example SoundFont".to_string(),
            version: "alpha".to_string(),
            description: Some("Just a good ol' example soundfont.".to_string()),
            authors: vec!["Pedro Braga <mrpedrobraga.com>".to_string()],
        },
        instruments: vec![Instrument {
            meta: InstrumentMetadata {
                name: "Sine Wave".to_string(),
                description: Some("Just a good ol' sine wave.".to_string()),
                categories: vec!["synth".to_string(), "chiptune".to_string()],
            },
            fragments: vec![AudioFragment::RawPCM(sample_1)],
            sampling_strategy: InstrumentSamplingStrategy::EuclideanVoronoi(
                EuclideanVoronoiSamplingStrategy {
                    dimensionality: 1,
                    dimension_names: vec!["note_pitch".to_string()],
                    points: vec![0],
                },
            ),
        }],
    };

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tests/soundfont.sft")
        .unwrap();
    pot::to_writer(&soundfont, &mut f).unwrap();
}

#[test]
fn load() {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open("./tests/soundfont.sft")
        .unwrap();
    let soundfont: Pack = pot::from_reader(f).unwrap();
    assert_eq!(soundfont.meta.name, "Example SoundFont");
}