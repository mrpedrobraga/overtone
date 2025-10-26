use std::marker::PhantomData;
use cables_core::graph::{Graph, Node};
use cables_macro::node_impl;

#[test]
fn pass_by_ref() {
    let mut graph = Graph::new();

    let a = graph.insert(Input(3.0f64));
    let by_ref = graph.insert(ByRef::<f64> { marker: PhantomData });

    graph.connect(a, 0, by_ref, 0).unwrap();
    
    let mut pip = graph.compile(by_ref, 0);
    pip.run();
}

struct Input<T>(T);

#[node_impl(fields(value = 0))]
impl<T: Copy + 'static> Node for Input<T> {
    fn process(out: &mut T) {
        *out = value;
    }
}

struct ByRef<T> {
    marker: PhantomData<T>,
}

#[node_impl]
impl<T: 'static> Node for ByRef<T> {
    fn process(input: &T, output: &mut &T) {
        *output = input
    }
}