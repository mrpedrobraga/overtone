use std::iter::Map;
use std::ops::Range;
use cables_core::graph::{Graph, Node};
use cables_macro::node_impl;

struct Source(Range<i32>);

#[node_impl(fields(value = 0))]
impl Node for Source {
    fn process(output: &mut Range<i32>) {
        *output = value.clone();
    }
}

struct Double;

#[node_impl]
impl Node for Double {
    fn process(input: &Range<i32>, output: &mut Map<Range<i32>, fn(i32)->i32>) {
        *output = input.clone().map(|x| x * 2)
    }
}

struct Sink;

#[node_impl]
impl Node for Sink {
    fn process(input: &mut Map<Range<i32>, fn(i32)->i32>) {
        println!("Sinkin' it up!");
        while let Some(val) = input.next() {
            println!("{}", val);
        }
    }
}

#[test]
fn monadic() {
    let mut graph = Graph::new();

    let source = graph.insert(Source(0..100));
    let double = graph.insert(Double);
    let sink = graph.insert(Sink);

    graph.connect(source, 0, double, 0).unwrap();
    graph.connect(double, 0, sink, 0).unwrap();

    let mut pip = graph.compile(sink, 0);
    pip.run();
}