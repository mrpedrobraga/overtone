use cables_core::graph::{Graph, Node, SocketData};
use cables_core::as_output;

fn main() {
    let mut graph = Graph::new();

    let sine = graph.insert(SineNode { frequency: 440.0 });

    let mut pip = graph.compile(sine, 0);
    pip.run();
}

struct SineNode {
    frequency: f32,
}
impl Node for SineNode {
    fn bind_parameters<'pip>(&self, parameters: &mut dyn Iterator<Item=*mut u8>) -> Box<dyn FnMut() + 'pip> {
        let out = as_output::<f64>(parameters.next().unwrap());
        Box::new(move || {
            *out = 42.0
        })
    }

    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        None
    }

    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        match socket_index {
            0 => Some(SocketData::new::<f64>()),
            _ => None
        }
    }
}