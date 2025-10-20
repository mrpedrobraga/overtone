use overtone::transformer::{NodeRef, Sink, Source, Value};
use std::any::Any;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use overtone::transformer::Node;

use nodes::combine::CombineNode;
use nodes::exporter_wav::WAVExporter;
use nodes::gain::GainNode;
use nodes::wave_generator::WaveGenerator;

pub mod audio;
pub mod nodes;

fn new_node<N: Node + 'static>(node: N) -> NodeRef {
    Arc::new(RwLock::new(node))
}

macro_rules! connect {
    ($a:expr, $a_out:expr, $b_in:expr, $b:expr) => {{
        $b.write().unwrap().connect($b_in, $a, $a_out.clone()).unwrap()
    }};
}

macro_rules! drain {
    ($a:expr) => {{
        ($a).write()
            .unwrap()
            .as_sink()
            .expect("Node could not be converted to a Sink.")
            .drain()
            .expect("Couldn't drain the sink.");
    }}
}

fn main() {
    let base = 261.63/2.0;

    let a = new_node(WaveGenerator::new(base));
    let b = new_node(WaveGenerator::new(base * 5.0 / 4.0));
    let c = new_node(WaveGenerator::new(base * 3.0 / 2.0));

    let ab = new_node(CombineNode::new());
    let abc = new_node(CombineNode::new());

    let master_gain = new_node(GainNode::new(0.25));

    let export_path = "./examples/production_graph/exports/";
    let wav_exporter = new_node(WAVExporter::new(export_path));

    connect!(  a, 0, 0,  ab);
    connect!(  b, 0, 1,  ab);
    connect!( ab, 0, 0, abc);
    connect!(  c, 0, 1, abc);
    connect!(abc, 0, 0, master_gain);
    connect!(master_gain, 0, 0, wav_exporter);

    drain!(wav_exporter);

    println!("Wrote result to '{}'.", export_path);
}