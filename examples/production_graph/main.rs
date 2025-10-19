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

    let n0 = new_node(WaveGenerator::new(base));
    let n1 = new_node(WaveGenerator::new(base * 5.0 / 4.0));
    let n2 = new_node(WaveGenerator::new(base * 3.0 / 2.0));

    let c1 = new_node(CombineNode::new());
    let c2 = new_node(CombineNode::new());

    let g1 = new_node(GainNode::new(0.25));

    let path = "./examples/production_graph/exports/";
    let nz = new_node(WAVExporter::new(path));

    connect!(n0, 0, 0, c1);
    connect!(n1, 0, 1, c1);
    connect!(c1, 0, 0, c2);
    connect!(n2, 0, 1, c2);
    connect!(c2, 0, 0, g1);
    connect!(g1, 0, 0, nz);

    drain!(nz);
    drain!(nz);
    drain!(nz);

    println!("Wrote result to '{}'.", path);
}