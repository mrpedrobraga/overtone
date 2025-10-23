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

pub trait Node {
    /// Returns a function that processes the node in
    /// terms of its parameters.
    fn bind(&self, inputs: &[*const u8], outputs: &[*mut u8]) -> Box<dyn FnMut()>;
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

            println!("NODE {:?}", node.0);

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
                .map(|(pointer, _)| {
                    println!("READS {:?}", &pointer);
                    pointer
                })
                .collect::<Vec<_>>();

            let mut output_ptrs: Vec<(*mut u8, SocketIndex)> = graph
                .edges
                .iter()
                .filter_map(
                    |(_, &(output_node, output_socket))| {
                        if output_node == node {
                            edge_pointers
                                .get(&(output_node, output_socket))
                                .map(|&p| (p, output_socket))
                        } else {
                            None
                        }
                    },
                )
                .collect();
            output_ptrs.sort_by_key(|(_, index)| *index);
            output_ptrs.dedup_by_key(|(_, index)| *index);
            let output_ptrs = output_ptrs
                .iter()
                .copied()
                .map(|(pointer, _)| {
                    println!("WRITES {:?}", &pointer);
                    pointer
                })
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
