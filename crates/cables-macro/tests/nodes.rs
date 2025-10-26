use cables_core::graph::Node;
use cables_core::graph::{Graph, SocketData};
use cables_macro::node_impl;

#[test]
fn test_node() {
    let mut graph = Graph::new();

    let a = graph.insert(Num { val: 3.0 });
    let b = graph.insert(Num { val: 6.0 });
    let ab = graph.insert(Add);
    let print = graph.insert(Print);

    graph.connect(a, 0, ab, 0).unwrap();
    graph.connect(b, 0, ab, 1).unwrap();
    graph.connect(ab, 0, print, 0).unwrap();

    let mut pip = graph.compile(print, 0);
    pip.run();
}

struct Add;
#[node_impl]
impl Node for Add {
    fn process(in1: &f32, in2: &f32, out: &mut f32) {
        *out = *in1 + *in2;
    }
}

struct Num {
    val: f32,
}
#[node_impl(fields(val))]
impl Node for Num {
    fn process(out: &mut f32) {
        *out = val;
    }
}

struct Print;
#[node_impl]
impl Node for Print {
    fn process(in1: &f32) {
        println!("{:?}", *in1);
    }
}
