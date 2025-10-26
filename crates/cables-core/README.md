# Cables

A library for creating directed acyclic graphs that will be executed thousands of times a second.
It's inspired by Blender's geometry nodes in which nodes may have multiple input and output sockets.

I created this implementation of Graph to be used for real-time audio processing. At 44100Hz, a standard
sample rate for audio, an entire graph like this needs to run under 22 microseconds.

Even though this graph was designed for audio, you can use it for anything else.
High sample rate never hertz. Hurts. Whatever.

```rust
fn main() {
    let mut graph = Graph::new();
    
    let a = graph.insert(Num::new(40.0));
    let b = graph.insert(Num::new(60.0));
    let ab = graph.insert(Sum);
    
    graph.connect(a, 0, ab, 0);
    graph.connect(b, 0, ab, 1);
   
    // Compile the graph into an efficient pipeline.
    // Compiling the graph, traversing it, checking the types
    // is slow (in order of microseconds).
    //
    // You need to call this every time the graph changes.
    let pipeline = graph.compile();
    
    // Running a compiled graph
    // is fast (in the order of nanoseconds).
    let result = pipeline.run();
}
```

You _will_ need to create your own nodes. This library doesn't come with any.

Nodes are effectively just functions, with a lot of reflection.

```rust
struct Num(pub f32);

#[node_impl(fields(num = 0))]
impl Node for Num {
    fn process(out: &mut f32) {
        *out = num
    }
}

struct Sum;

#[node_impl]
impl Node for Sum {
    fn process(&self, a: &f32, b: &f32, out: &mut f32) {
        *out = *a + *b;
    }
}
```

## Licensing

MIT