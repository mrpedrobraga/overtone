#![allow(unused)]
use std::{collections::HashMap, time::Instant};

/// Function that represents a node's processing.
///
/// When running, it will resolve and cast the pointers into the proper input and output types!
type NodeFunc = fn(inputs: &[*const u8], outputs: &[*mut u8]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct NodeKey(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct SocketIndex(usize);

struct Node;

struct Graph {
    nodes: HashMap<NodeKey, Node>,
    // The key is some node's input socket, the value is some node's output socket
    // Data flows in this direction   <----
    // But it's a pull model so we hash the output.
    edges: HashMap<(NodeKey, SocketIndex), (NodeKey, SocketIndex)>,
    next_id: usize,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_id: 0,
        }
    }

    /// Inserts a new node in the graph — the node is now owned by the graph.
    ///
    /// The function returns a key which can be used to access the node temporarily.
    fn insert(&mut self, node: Node) -> NodeKey {
        let index = self.next_id;
        self.next_id += 1;
        self.nodes.insert(NodeKey(index), node);
        NodeKey(index)
    }

    /// Draws a connection from a node's output socket to another node's input socket.
    fn connect(
        &mut self,
        output_node: NodeKey,
        output_socket: usize,
        input_node: NodeKey,
        input_socket: usize,
    ) -> Result<(), ()> {
        self.edges.insert(
            (output_node, SocketIndex(output_socket)),
            (input_node, SocketIndex(input_socket)),
        );
        Ok(())
    }

    /// Compiles this graph (from the perspective of a sink)
    /// so it can be executed thousands a time a second.
    fn compile(&self, sink: NodeKey) -> GraphPipeline {
        GraphPipeline::from_graph(self, sink)
    }
}

struct GraphPipeline {
    /// Contains one allocation per graph edge.
    /// Shared memory space that the nodes use to do work.
    edge_data: Vec<u8>,

    /// Contains a list of functions, one for each node of the graph
    /// in topological order, i.e., dependency order.
    ///
    /// So it's safe from a function to read from anywhere in `edge_data`,
    /// since it will always be ordered _after_ a function that set a value there.
    vertices: Vec<Box<dyn FnMut()>>,
}

impl GraphPipeline {
    fn from_graph(graph: &Graph, sink: NodeKey) -> Self {
        let mut pip = Self {
            edge_data: Vec::new(),
            vertices: Vec::new(),
        };

        // HERE IS WHERE THE MAGIC HAPPENS

        pip
    }

    /// Runs the pipeline.
    ///
    /// This function is incredibly fast.
    ///
    /// The only overhead is one pointer dereference to get the closure (since it's boxed),
    /// and then one pointer dereference per input or output.
    ///
    /// It doesn't get much better than this without source code or AST compilation.
    ///
    /// # Mutability
    ///
    /// Not sure if this _should_ be mutable. I guess since nodes have state that mutates
    /// when this runs, yeah, sure.
    fn run(&mut self) {
        for vertex in self.vertices.iter_mut() {
            vertex()
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    let a = graph.insert(Node);
    let b = graph.insert(Node);
    let c = graph.insert(Node);
    let d = graph.insert(Node);

    graph.connect(a, 0, c, 0).unwrap();
    graph.connect(b, 0, c, 1).unwrap();
    graph.connect(c, 0, d, 0).unwrap();

    let mut pipeline = graph.compile(d);
    pipeline.run();
}

/// The code above should effectively do this
/// automatically — using values, associated types, traits, etc;
fn handmade_model() {
    let mut edge_data: Vec<u8> = Vec::new();

    edge_data.extend_from_slice(&[0, 0, 0, 0]);
    edge_data.extend_from_slice(&[0, 0, 0, 0]);
    edge_data.extend_from_slice(&[0, 0, 0, 0]);

    let edge_data_ptr = edge_data.as_mut_ptr();

    let mut vertices: Vec<Box<dyn FnMut()>> = Vec::new();

    vertices.push(Box::new(|| {
        proc_a(edge_data_ptr.wrapping_add(0));
    }));
    vertices.push(Box::new(|| {
        proc_b(edge_data_ptr.wrapping_add(8));
    }));
    vertices.push(Box::new(|| {
        proc_c(
            // The variable parameter count will be hard to pull off...
            // There is one parameter for each input or output edge.
            edge_data_ptr.wrapping_add(0),
            edge_data_ptr.wrapping_add(8),
            edge_data_ptr.wrapping_add(12),
        );
    }));

    let amount = 10000;
    let before = Instant::now();
    for _ in 0..amount {
        for process_vertex in vertices.iter_mut() {
            process_vertex();
        }
    }
    println!("After = {:?}", before.elapsed().div_f32(amount as f32));

    let result = as_ref::<f32>(edge_data_ptr.wrapping_add(12));
    dbg!(result);
}

fn proc_a(out0: *mut u8) {
    let out0 = as_mut_ref::<f32>(out0);

    *out0 = 1.0;
}

fn proc_b(out0: *mut u8) {
    let out0 = as_mut_ref::<f32>(out0);

    *out0 = 2.0;
}

fn proc_c(in0: *const u8, in1: *const u8, out0: *mut u8) {
    let in0 = as_ref::<f32>(in0);
    let in1 = as_ref::<f32>(in1);
    let out0 = as_mut_ref::<f32>(out0);

    *out0 = *in0 + *in1;
}

#[inline]
pub fn as_ref<'a, T>(ptr: *const u8) -> &'a T {
    unsafe { &*ptr.cast::<T>() }
}

#[inline]
pub fn as_mut_ref<'a, T>(ptr: *mut u8) -> &'a mut T {
    unsafe { &mut *ptr.cast::<T>() }
}
