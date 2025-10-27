#![allow(unused)]
use std::{
    hash::Hash,
    io::Write,
    time::Instant,
};
use std::hint::black_box;
use std::ptr::NonNull;
use graph::Graph;
use nodes::{NumSource, Sum, YellNum};
use crate::graph::{GraphPipeline, Node};
use crate::nodes::Double;

pub mod old;
pub mod graph;
pub mod nodes;

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

    let mut pipeline = graph.compile(d, 0);

    let iterations = 1000;
    let before = black_box(Instant::now());
    for _ in 0..iterations {
        pipeline.run();
    }
    println!("Took {:?}", before.elapsed().div_f64(iterations as f64));
    std::io::stdout().flush().unwrap();
}

#[test]
pub fn compile_speed() {
    let mut graph = Graph::new();

    let a = graph.insert(NumSource { value: 2.0 });
    let b = graph.insert(NumSource { value: 1.0 });
    let c = graph.insert(Sum);
    let d = graph.insert(YellNum);

    graph.connect(a, 0, c, 0).unwrap();
    graph.connect(b, 0, c, 1).unwrap();
    graph.connect(c, 0, d, 0).unwrap();

    let iterations = 100;
    let before = Instant::now();
    for _ in 0..iterations {
        let mut pipeline = graph.compile(d, 0);
    }
    println!("Took {:?}", before.elapsed().div_f64(iterations as f64));
    std::io::stdout().flush().unwrap();
}

#[test]
fn diamond() {
    let mut graph = Graph::new();

    //      ┌────────┐
    //      │ NumSrc │0
    //      └───┬────┘
    //          │
    //     ┌────┴────┐
    //     │         │
    // ┌───▼───┐ ┌───▼───┐
    // │Double │1│Double │2
    // └───┬───┘ └───┬───┘
    //     │         │
    //     └────┬────┘
    //        ┌─▼─┐
    //        │Sum│3
    //        └─┬─┘
    //          │
    //      ┌───▼───┐
    //      │YellNum│4
    //      └───────┘

    let source = graph.insert(NumSource { value: 3.5 });
    let doubler_l = graph.insert(Double);
    let doubler_r = graph.insert(Double);
    let sum = graph.insert(Sum);
    let output = graph.insert(YellNum);

    graph.connect(source, 0, doubler_l, 0).unwrap();
    graph.connect(source, 0, doubler_r, 0).unwrap();
    graph.connect(doubler_l, 0, sum, 0).unwrap();
    graph.connect(doubler_r, 0, sum, 1).unwrap();
    graph.connect(sum, 0, output, 0).unwrap();

    let mut p = GraphPipeline::from_graph(&graph, output, 0);

    let iterations = 44100;
    let before = Instant::now();
    for _ in 0..iterations {
        black_box(p.run());
    }
    let elapsed = before.elapsed();
    println!("Total {:?} ({:?} per run)", elapsed, elapsed.div_f64(iterations as f64));
    std::io::stdout().flush().unwrap();
}

#[inline]
pub fn as_input<'a, T>(ptr: NonNull<u8>) -> &'a T {
    unsafe { ptr.cast::<T>().as_ref() }
}

#[inline]
pub fn as_output<'a, T>(ptr: NonNull<u8>) -> &'a mut T {
    unsafe { ptr.cast::<T>().as_mut() }
}