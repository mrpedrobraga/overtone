use std::any::TypeId;
use std::collections::{HashMap, HashSet};
/// Function that represents a node's processing.
///
/// When running, it will resolve and cast the pointers into the proper input and output types!
pub type NodeFunc = fn(inputs: &[*const u8], outputs: &[*mut u8]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeKey(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SocketIndex(usize);

pub struct SocketData {
    size: usize,
    align: usize,
}
impl SocketData {
    pub fn new<T: 'static>() -> Self {
        Self {
            size: size_of::<T>(),
            align: align_of::<T>(),
        }
    }
}

pub trait Node {
    /// Returns a function that processes the node in terms of its parameters.
    ///
    /// It takes self here simply to be dyn-compatible.
    ///
    /// TODO: Maybe instead of returning a boxed closure, which is going to be
    /// put in a Vec anyways, maybe pass an arena for `bind_parameters` to allocate the closure in.
    fn bind_parameters(&self, parameters: &mut dyn Iterator<Item = *mut u8>) -> Box<dyn FnMut()>;

    /// Returns data about an input socket.
    /// Take self so the trait is dyn-compatible.
    fn input_socket(&self, socket_index: usize) -> Option<SocketData>;

    /// Returns data about an output socket.
    /// Take self so the trait is dyn-compatible.
    fn output_socket(&self, socket_index: usize) -> Option<SocketData>;
}

pub struct Graph {
    nodes: HashMap<NodeKey, Box<dyn Node>>,
    // The key is some node's input socket, the value is some node's output socket
    // Data flows in this direction   <----
    // But it's a pull model so we hash the input.
    edges: HashMap<(NodeKey, SocketIndex), (NodeKey, SocketIndex)>,
    next_id: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_id: 0,
        }
    }

    /// Inserts a new node in the graph — the node is now owned by the graph.
    ///
    /// The function returns a key which can be used to access the node temporarily.
    pub fn insert<N: Node + 'static>(&mut self, node: N) -> NodeKey {
        let index = self.next_id;
        self.next_id += 1;
        self.nodes.insert(NodeKey(index), Box::new(node));
        NodeKey(index)
    }

    /// Draws a connection from a node's output socket to another node's input socket.
    ///
    /// # Usage
    ///
    /// Please realize that data flows from the output of a node to the input of the other node.
    pub fn connect(
        &mut self,
        output_node: NodeKey,
        output_socket: usize,
        input_node: NodeKey,
        input_socket: usize,
    ) -> Result<(), ()> {
        self.edges.insert(
            (input_node, SocketIndex(input_socket)),
            (output_node, SocketIndex(output_socket)),
        );
        Ok(())
    }

    /// Compiles this graph (from the perspective of a sink)
    /// so it can be executed thousands a time a second.
    pub fn compile(&self, sink: NodeKey, sink_socket: usize) -> GraphPipeline {
        GraphPipeline::from_graph(self, sink, sink_socket)
    }
}

pub struct GraphPipeline {
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
    pub fn from_graph(graph: &Graph, sink_node: NodeKey, sink_socket: usize) -> Self {
        let mut edge_data = Vec::with_capacity(256);
        let mut vertices = Vec::with_capacity(graph.nodes.len());
        let mut visited_nodes = HashSet::<NodeKey>::new();
        let mut socket_pointers = HashMap::<(NodeKey, SocketIndex), *mut u8>::new();

        fn depth_first_traverse(
            current_node_key: NodeKey,
            current_output_socket: SocketIndex,
            graph: &Graph,
            visited_nodes: &mut HashSet<NodeKey>,
            edge_data: &mut Vec<u8>,
            socket_pointer_cache: &mut HashMap<(NodeKey, SocketIndex), *mut u8>,
            vertices: &mut Vec<Box<dyn FnMut()>>,
        ) -> Option<*mut u8> {
            if visited_nodes.contains(&current_node_key) {
                return Some(
                    socket_pointer_cache
                        .get(&(current_node_key, current_output_socket))
                        .copied()
                        .unwrap(),
                );
            }
            visited_nodes.insert(current_node_key);

            /// TODO: Sort the edges based on the current node's socket order.
            let input_pointers = graph
                .edges
                .iter()
                .filter(|((to_node, _), _)| *to_node == current_node_key)
                .map(|(to, from)| {
                    let input_pointer = depth_first_traverse(
                        from.0,
                        from.1,
                        graph,
                        visited_nodes,
                        edge_data,
                        socket_pointer_cache,
                        vertices,
                    )
                    .unwrap();

                    input_pointer
                })
                .collect::<Vec<_>>();

            let current_node = graph.nodes.get(&current_node_key).unwrap();

            let mut output_pointer_to_return = None;

            let output_pointers = (0..)
                .map_while(|output_index| {
                    current_node
                        .output_socket(output_index)
                        .map(|output| (output_index, output))
                })
                .map(|(output_index, output)| {
                    let pointer = allocate_aligned(edge_data, output.size, output.align);
                    socket_pointer_cache
                        .insert((current_node_key, SocketIndex(output_index)), pointer);

                    if output_index == current_output_socket.0 {
                        output_pointer_to_return = Some(pointer);
                    }

                    pointer
                });

            let mut parameter_iterator = input_pointers.into_iter().chain(output_pointers);
            let vertex = current_node.bind_parameters(&mut parameter_iterator);
            vertices.push(vertex);
            
            output_pointer_to_return
        }

        let sink_output_pointer = depth_first_traverse(
            sink_node,
            SocketIndex(sink_socket),
            graph,
            &mut visited_nodes,
            &mut edge_data,
            &mut socket_pointers,
            &mut vertices,
        );

        Self {
            edge_data,
            vertices,
        }
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
    pub fn run(&mut self) {
        for vertex in self.vertices.iter_mut() {
            vertex()
        }
    }
}

fn allocate_aligned(buffer: &mut Vec<u8>, size: usize, align: usize) -> *mut u8 {
    let current_pointer = buffer.as_mut_ptr().wrapping_byte_add(buffer.len());
    let padding = align - current_pointer as usize % align;
    let current_pointer = current_pointer.wrapping_add(padding);
    buffer.extend(std::iter::repeat(0x00).take(padding + size));
    current_pointer
}