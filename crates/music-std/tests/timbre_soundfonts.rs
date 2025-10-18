use overtone_music_std::formats::pcm::AudioPcm;
use overtone_music_std::formats::timbre::*;
use std::io::Write;

#[test]
fn store() {
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
            fragments: vec![AudioFragment::RawPCM(AudioPcm::example())],
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
