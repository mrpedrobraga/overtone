# AudioGraph

The implementation of Production Graphs has a few particular requirements:

- Graphs must be:
    - Mutable;
        - Some graph draining tasks (like exporting) will lock the graph for big chunks of time;
        - Other tasks like previewing will lock the graph for little chunks of time, allowing smooth editing;
- Nodes may have many input and output sockets;
    - A single output socket can connect to many input sockets;
    - An input socket can receive a single connection;
    - When two sockets are connected, the connection is validated once, and then it's safe to pull data through at audio rates (thousands of time per second);
- Nodes may have internal state;
    - Some state associated with the node should be serialized... think audio plugin sliders and knobs. But these will probably be coded as inputs;
    - Internal state (which will be mutated by executing the graph) need not be serializable;