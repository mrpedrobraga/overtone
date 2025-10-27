// src/main.rs
use bumpalo::Bump;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeKey(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SocketIndex(usize);

/// Factory function pointer that allocates a `T::default()` in the arena
/// and returns a raw pointer to it as `*mut dyn Any`.
type FactoryFn = fn(&Bump) -> *mut dyn Any;

pub struct SocketData {
    pub type_id: TypeId,
    pub factory: FactoryFn,
}

impl SocketData {
    pub fn of<T: Any + Default + 'static>() -> Self {
        fn alloc_t<T: Any + Default + 'static>(bump: &Bump) -> *mut dyn Any {
            // allocate T in the arena and return a raw pointer to it as dyn Any
            // bumpalo::Bump::alloc returns &'a mut T; we cast that into *mut dyn Any
            let r: &mut T = bump.alloc(T::default());
            r as &mut dyn Any as *mut dyn Any
        }
        Self {
            type_id: TypeId::of::<T>(),
            factory: alloc_t::<T>,
        }
    }
}

pub trait Node {
    /// receive an iterator of `&'pip mut dyn Any` (inputs then outputs).
    fn bind_parameters<'pip>(
        &self,
        parameters: &mut dyn Iterator<Item = &'pip mut dyn Any>,
    ) -> Box<dyn FnMut() + 'pip>;

    fn input_socket(&self, socket_index: usize) -> Option<SocketData>;
    fn output_socket(&self, socket_index: usize) -> Option<SocketData>;
}

pub struct Graph {
    nodes: HashMap<NodeKey, Box<dyn Node>>,
    // edges: (to_node, to_socket) -> (from_node, from_socket)
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
    pub fn insert<N: Node + 'static>(&mut self, node: N) -> NodeKey {
        let idx = self.next_id;
        self.next_id += 1;
        self.nodes.insert(NodeKey(idx), Box::new(node));
        NodeKey(idx)
    }
    pub fn connect(
        &mut self,
        output_node: NodeKey,
        output_socket: usize,
        input_node: NodeKey,
        input_socket: usize,
    ) {
        self.edges.insert(
            (input_node, SocketIndex(input_socket)),
            (output_node, SocketIndex(output_socket)),
        );
    }
    pub fn compile(&self, sink: NodeKey, sink_socket: usize) -> GraphPipeline<'_> {
        GraphPipeline::from_graph(self, sink, sink_socket)
    }
}

/// The pipeline owns the arena and the compiled closures.
/// All socket storage is allocated in the arena. Closures borrow `&'a mut dyn Any`.
pub struct GraphPipeline<'pip> {
    _arena: Bump,
    vertices: Vec<Box<dyn FnMut() + 'pip>>,
    // keep map only for debug/possible post-inspection, optional
    _socket_map: HashMap<(NodeKey, SocketIndex), *mut dyn Any>,
}

impl<'pip> GraphPipeline<'pip> {
    pub fn from_graph(graph: &Graph, sink_node: NodeKey, sink_socket: usize) -> Self {
        let mut pipeline = Self {
            _arena: Bump::new(),
            vertices: Vec::with_capacity(graph.nodes.len()),
            _socket_map: HashMap::new(),
        };
        
        let arena = &mut pipeline._arena;
        let mut vertices = &mut pipeline.vertices;
        let mut socket_pointer_cache = &mut pipeline._socket_map;

        let mut visited: HashSet<NodeKey> = HashSet::new();

        // recursive DFS returns pointer index (raw pointer) to the requested node's output socket
        fn dfs<'a>(
            current_node_key: NodeKey,
            current_output_socket: SocketIndex,
            graph: &Graph,
            visited: &mut HashSet<NodeKey>,
            arena: &Bump,
            socket_pointer_cache: &mut HashMap<(NodeKey, SocketIndex), *mut dyn Any>,
            vertices: &mut Vec<Box<dyn FnMut() + 'a>>,
        ) -> *mut dyn Any {
            if let Some(&ptr) = socket_pointer_cache.get(&(current_node_key, current_output_socket))
            {
                return ptr;
            }
            if visited.contains(&current_node_key) {
                // if visited but pointer was not present something is wrong,
                // but return the pointer if present (checked above).
            }
            visited.insert(current_node_key);

            // build inputs: for every edge (to_node == current_node_key) traverse the producer
            let input_ptrs: Vec<*mut dyn Any> = graph
                .edges
                .iter()
                .filter(|((to_node, _), _)| *to_node == current_node_key)
                .map(|((_to, _to_socket), from)| {
                    let from_node = from.0;
                    let from_socket = from.1;
                    dfs(
                        from_node,
                        from_socket,
                        graph,
                        visited,
                        arena,
                        socket_pointer_cache,
                        vertices,
                    )
                })
                .collect();

            // allocate outputs for this node and store pointers in cache
            let node = graph.nodes.get(&current_node_key).expect("node exists");
            // collect output sockets until None
            let mut output_ptrs: Vec<*mut dyn Any> = Vec::new();
            for out_idx in 0usize.. {
                if let Some(sock) = node.output_socket(out_idx) {
                    // allocate via the factory; factory returns a *mut dyn Any pointing inside arena
                    let p = (sock.factory)(arena);
                    socket_pointer_cache.insert((current_node_key, SocketIndex(out_idx)), p);
                    output_ptrs.push(p);
                } else {
                    break;
                }
            }

            // Build a vector of mutable references (&'a mut dyn Any) to pass into bind_parameters.
            // SAFETY: Convert raw pointers to &'a mut dyn Any here. This is the only unsafe.
            // It's sound because:
            //  - `arena` owns all allocations and outlives the compiled closures ('a).
            //  - We allocated each pointer uniquely and will not mutate aliasingly during compilation.
            //  - Closures borrow these references for 'a. They only run later, single-threaded.
            let mut params_refs: Vec<&'a mut dyn Any> = Vec::with_capacity(
                input_ptrs.len() + output_ptrs.len(),
            );

            for &p in &input_ptrs {
                let r: &'a mut dyn Any = unsafe { &mut *p };
                params_refs.push(r);
            }
            for &p in &output_ptrs {
                let r: &'a mut dyn Any = unsafe { &mut *p };
                params_refs.push(r);
            }

            // Create an iterator over &'a mut dyn Any and pass to bind_parameters.
            // Note: we create an iterator over the params_refs vector elements.
            let mut iter = params_refs.into_iter();
            let vertex = node.bind_parameters(&mut iter);
            vertices.push(vertex);

            // return pointer for requested output socket
            socket_pointer_cache
                .get(&(current_node_key, current_output_socket))
                .copied()
                .expect("output pointer must be present")
        }

        dfs(
            sink_node,
            SocketIndex(sink_socket),
            graph,
            &mut visited,
            &arena,
            &mut socket_pointer_cache,
            &mut vertices,
        );

        pipeline
    }

    pub fn run(&mut self) {
        for v in self.vertices.iter_mut() {
            v()
        }
    }
}

// ---------------------- Example nodes --------------------------

struct ConstF32(pub f32);
impl Node for ConstF32 {
    fn input_socket(&self, _socket_index: usize) -> Option<SocketData> {
        None
    }
    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        if socket_index == 0 {
            Some(SocketData::of::<f32>())
        } else {
            None
        }
    }
    fn bind_parameters<'pip>(
        &self,
        parameters: &mut dyn Iterator<Item = &'pip mut dyn Any>,
    ) -> Box<dyn FnMut() + 'pip> {
        // no inputs, one output
        let out_any = parameters.next().unwrap();
        // runtime check to help debugging
        //assert_eq!(out_any.type_id(), TypeId::of::<f32>());
        let val = self.0;
        Box::new(move || {
            let out = out_any.downcast_mut::<f32>().unwrap();
            *out = val;
        })
    }
}

struct AddF32;
impl Node for AddF32 {
    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        if socket_index < 2 {
            Some(SocketData::of::<f32>())
        } else {
            None
        }
    }
    fn output_socket(&self, socket_index: usize) -> Option<SocketData> {
        if socket_index == 0 {
            Some(SocketData::of::<f32>())
        } else {
            None
        }
    }
    fn bind_parameters<'pip>(
        &self,
        parameters: &mut dyn Iterator<Item = &'pip mut dyn Any>,
    ) -> Box<dyn FnMut() + 'pip> {
        let a_any = parameters.next().unwrap();
        let b_any = parameters.next().unwrap();
        let out_any = parameters.next().unwrap();

        //assert_eq!(a_any.type_id(), TypeId::of::<f32>());
        //assert_eq!(b_any.type_id(), TypeId::of::<f32>());
        //assert_eq!(out_any.type_id(), TypeId::of::<f32>());

        // capture references; these are &'pip mut dyn Any where 'pip == 'a
        Box::new(move || {
            let a = a_any.downcast_ref::<f32>().unwrap();
            let b = b_any.downcast_ref::<f32>().unwrap();
            let out = out_any.downcast_mut::<f32>().unwrap();
            *out = *a + *b;
        })
    }
}

struct PrintF32(&'static str);
impl Node for PrintF32 {
    fn input_socket(&self, socket_index: usize) -> Option<SocketData> {
        if socket_index == 0 {
            Some(SocketData::of::<f32>())
        } else {
            None
        }
    }
    fn output_socket(&self, _socket_index: usize) -> Option<SocketData> {
        None
    }
    fn bind_parameters<'pip>(
        &self,
        parameters: &mut dyn Iterator<Item = &'pip mut dyn Any>,
    ) -> Box<dyn FnMut() + 'pip> {
        let in_any = parameters.next().unwrap();
        //assert_eq!(in_any.type_id(), TypeId::of::<f32>());
        let tag = self.0;
        Box::new(move || {
            let v = in_any.downcast_ref::<f32>().unwrap();
            println!("{} = {}", tag, v);
        })
    }
}

// ---------------------- Demo --------------------------

fn main() {
    let mut g = Graph::new();

    let c1 = g.insert(ConstF32(1.5));
    let c2 = g.insert(ConstF32(2.25));
    let add = g.insert(AddF32);
    let print = g.insert(PrintF32("result"));

    g.connect(c1, 0, add, 0);
    g.connect(c2, 0, add, 1);
    g.connect(add, 0, print, 0);

    let mut pipeline = g.compile(print, 0);
    pipeline.run();
}
