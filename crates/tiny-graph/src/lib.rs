#![allow(unused)]
use std::{
    hash::Hash,
    io::Write,
    time::Instant,
};
use std::marker::PhantomData;
use graph::Graph;
use nodes::{NumSource, Sum, YellNum};
use crate::graph::Node;
use crate::nodes::{as_input, as_output, Double};

pub mod old;
mod graph;
mod nodes;

#[test]
pub fn speed() {
    let mut graph = Graph::new();

    let a = graph.insert(NumSource { value: 2.0 });
    let b = graph.insert(NumSource { value: 1.0 });
    let c = graph.insert(Sum);
    let d = graph.insert(YellNum);

    graph.connect(a, 0, c, 0).unwrap();
    graph.connect(b, 0, c, 1).unwrap();
    graph.connect(c, 0, d, 0).unwrap();

    let mut pipeline = graph.compile(d);

    let iterations = 1000;
    let before = Instant::now();
    for _ in 0..iterations {
        pipeline.run();
    }
    println!("Took {:?}", before.elapsed().div_f64(iterations as f64));
    std::io::stdout().flush().unwrap();
}

#[test]
pub fn diamond () {
    let mut graph = Graph::new();

    let source = graph.insert(NumSource { value: 2.0 });
    //let splitter = graph.insert(Split::<f64>::default());
    let doubler_l = graph.insert(Double);
    let doubler_r = graph.insert(Double);
    let sum = graph.insert(Sum);
    let output = graph.insert(YellNum);

    graph.connect(source, 0, doubler_l, 0).unwrap();
    graph.connect(source, 0, doubler_r, 0).unwrap();
    graph.connect(doubler_l, 0, sum, 0).unwrap();
    graph.connect(doubler_r, 0, sum, 1).unwrap();
    graph.connect(sum, 0, output, 0).unwrap();

    let mut p = graph.compile(output);
    p.run();


}