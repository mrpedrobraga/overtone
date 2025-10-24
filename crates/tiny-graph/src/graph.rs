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
    type_id: TypeId,
    size: usize,
}
impl SocketData {
    pub fn new<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            size: size_of::<T>(),
        }
    }
}

pub trait Node {
    /// Returns a function that processes the node in
    /// terms of its parameters.
    ///
    /// It takes self here simply to be dyn-compatible.
    fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()>;

    /// Returns a function that processes the node in terms of its parameters.
    ///
    /// It takes self here simply to be dyn-compatible.
    ///
    /// TODO: Maybe instead of returning a boxed closure, which is going to be
    /// put in a Vec anyways, maybe pass an arena for `bind` to allocate the closure in.
    fn bind2(&self, parameters: &mut dyn Iterator<Item = *mut u8>) -> Box<dyn FnMut()>;

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
    pub fn compile(&self, sink: NodeKey) -> GraphPipeline {
        GraphPipeline::from_graph(self, sink)
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
    pub fn from_graph2(graph: &Graph, sink_node: NodeKey, sink_socket: usize) -> Self {
        /// Precalculate the total amount of output sockets.
        /// It's important that this vector never reallocates after we
        /// start taking pointers from it;
        let mut edge_data = Vec::with_capacity(128);
        fn allocate<T>(buffer: &mut Vec<u8>) -> *mut u8 {
            let index = buffer.len();
            buffer.extend(std::iter::repeat_n(0x00, size_of::<T>()));
            (&mut buffer[index]) as *mut u8
        }

        /// This vector can reallocate, but it's better if it doesn't, right?
        /// Not all nodes in the graph will be part of the pipeline but this is the
        /// only upper bound we get before we start traversing.
        let mut vertices = Vec::with_capacity(graph.nodes.len());

        /// Stores all the nodes we've already visited.
        let mut visited_nodes = HashSet::<NodeKey>::new();
        /// Pointers for all the output sockets' memory regions.
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
                println!("Node {} already visited: going back.", current_node_key.0);
                return Some(
                    socket_pointer_cache
                        .get(&(current_node_key, current_output_socket))
                        .copied()
                        .unwrap(),
                );
            }
            visited_nodes.insert(current_node_key);

            println!("Pushed node {} onto the stack.", current_node_key.0);

            /// TODO: Sort the edges based on the current node's socket order.
            let input_pointers = graph
                .edges
                .iter()
                .filter(|((to_node, _), _)| *to_node == current_node_key)
                .map(|(to, from)| {
                    println!(
                        "Walking through {}(p{}) -> {}(p{})",
                        to.0.0, to.1.0, from.0.0, from.1.0
                    );

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

                    println!(
                        "Walking through {}(p{}) -> {}(p{}) with pointer {:?}",
                        from.0.0, from.1.0, to.0.0, to.1.0, input_pointer
                    );

                    input_pointer
                })
                .collect::<Vec<_>>();

            println!(
                "Node {}'s dependencies are calculated, we can allocate it.",
                current_node_key.0
            );
            let current_node = graph.nodes.get(&current_node_key).unwrap();

            let mut output_pointer_to_return = None;

            let output_pointers = (0..).scan((), move |_, output_index| {
                if let Some(output) = current_node.output_socket(output_index) {
                    let pointer = allocate::<f32>(edge_data);
                    socket_pointer_cache
                        .insert((current_node_key, SocketIndex(output_index)), pointer);

                    if output_index == current_output_socket.0 {
                        output_pointer_to_return = Some(pointer);
                    }

                    println!(
                        "- Allocating output {} with {} bytes.",
                        output_index, output.size
                    );
                    Some(pointer)
                } else {
                    None
                }
            });

            println!(
                "Allocating node {} and popping it from stack.",
                current_node_key.0
            );
            let mut parameter_iterator = input_pointers.into_iter().chain(output_pointers);
            let vertex = current_node.bind2(&mut parameter_iterator);
            vertices.push(vertex);
            /// Safety: The loop above will definitely set the value,
            /// since it does so within an infinite loop,
            /// within a condition that we know is true
            ///  (an output exists with the same index in `current_output_socket`),
            /// since it's how `depth_first_traverse` was called in the first place.
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

    /// Creates a pipeline given a graph — notice this takes a & reference to the graph,
    /// and thus does not mutate it.
    pub fn from_graph(graph: &Graph, sink: NodeKey) -> Self {
        let mut pip = Self {
            edge_data: Vec::with_capacity(128 /* Total amount */),
            vertices: Vec::new(),
        };

        let mut visited_nodes = HashSet::<NodeKey>::new();
        let mut edge_pointers: HashMap<(NodeKey, SocketIndex), *mut u8> = HashMap::new();

        for (&(input_node, input_socket), &(output_node, output_socket)) in graph.edges.iter() {
            let edge_position = pip.edge_data.len();
            // Instead of pushing like this, allocate based on the edge's type's length.
            pip.edge_data.extend(std::iter::repeat_n(0x00, 8));
            edge_pointers.insert(
                (output_node, output_socket),
                pip.edge_data.as_mut_ptr().wrapping_byte_add(edge_position),
            );
        }

        fn traverse(
            graph: &Graph,
            node: NodeKey,
            visited: &mut HashSet<NodeKey>,
            edge_data: &mut Vec<u8>,
            edge_pointers: &mut HashMap<(NodeKey, SocketIndex), *mut u8>,
            vertices: &mut Vec<Box<dyn FnMut()>>,
        ) {
            if visited.contains(&node) {
                return;
            }
            visited.insert(node);

            for (&(input_node, input_socket), &(output_node, output_socket)) in graph.edges.iter() {
                if (input_node == node) {
                    traverse(
                        graph,
                        output_node,
                        visited,
                        edge_data,
                        edge_pointers,
                        vertices,
                    );
                }
            }

            let mut input_ptrs: Vec<(*const u8, SocketIndex)> = graph
                .edges
                .iter()
                .filter_map(|(&(input_node, input_socket), output)| {
                    if input_node == node {
                        edge_pointers
                            .get(output)
                            .map(|&p| (p as *const u8, input_socket))
                    } else {
                        None
                    }
                })
                .collect();
            input_ptrs.sort_by_key(|(_, index)| *index);
            let input_ptrs = input_ptrs
                .iter()
                .copied()
                .map(|(pointer, _)| pointer)
                .collect::<Vec<_>>();

            let mut output_ptrs: Vec<(*mut u8, SocketIndex)> = graph
                .edges
                .iter()
                .filter_map(|(_, &(output_node, output_socket))| {
                    if output_node == node {
                        edge_pointers
                            .get(&(output_node, output_socket))
                            .map(|&p| (p, output_socket))
                    } else {
                        None
                    }
                })
                .collect();
            output_ptrs.sort_by_key(|(_, index)| *index);
            output_ptrs.dedup_by_key(|(_, index)| *index);
            let output_ptrs = output_ptrs
                .iter()
                .copied()
                .map(|(pointer, _)| pointer)
                .collect::<Vec<_>>();

            // When it's time to bind a node's function, we give it
            // two slices `&[*const u8]` `&[*mut u8]`.
            // Then we call its implementation of `Node::bind(inputs: &[*const u8], outputs: &[*mut u8])`
            if let Some(node_obj) = graph.nodes.get(&node) {
                vertices.push(node_obj.bind(&input_ptrs, &output_ptrs));
            }
        }

        traverse(
            graph,
            sink,
            &mut visited_nodes,
            &mut pip.edge_data,
            &mut edge_pointers,
            &mut pip.vertices,
        );

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
    pub fn run(&mut self) {
        for vertex in self.vertices.iter_mut() {
            vertex()
        }
    }
}
