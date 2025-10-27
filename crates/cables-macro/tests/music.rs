use std::f32::consts::TAU;
use std::iter::Map;
use std::ops::Range;
use cables_core::graph::{Graph, Node};
use cables_macro::node_impl;

struct Tick(Range<i32>);

#[node_impl(fields(value = 0))]
impl Node for Tick {
    fn process(output: &mut Range<i32>) {
        *output = value.clone();
    }
}

struct Sine;

#[node_impl]
impl Node for Sine {
    fn process(input: &Range<i32>, output: &mut Map<Range<i32>, fn(i32)->f32>) {
        *output = input.clone().map(|tick| {
            let t = tick as f32 / 44100.0;
            (440.0 * TAU * t).sin()
        });
    }
}

struct Sink;

#[node_impl]
impl Node for Sink {
    fn process(input: &mut Map<Range<i32>, fn(i32)->f32>) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer =
            hound::WavWriter::create("tests/monadic-sine.wav", spec).expect("Failed to create WAV file");

        for sample in input.clone() {
            let scaled = (sample * i16::MAX as f32) as i16;
            writer.write_sample(scaled).unwrap();
        }

        writer.finalize().unwrap();
    }
}

#[test]
fn monadic() {
    let mut graph = Graph::new();

    let ticker = graph.insert(Tick(0..44100));
    let sine = graph.insert(Sine);
    let sink = graph.insert(Sink);

    graph.connect(ticker, 0, sine, 0).unwrap();
    graph.connect(sine, 0, sink, 0).unwrap();

    let mut pip = graph.compile(sink, 0);
    pip.run();
}