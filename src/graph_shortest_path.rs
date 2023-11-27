use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Node<T> {
    value: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Edge<T> {
    outbound: Node<T>,
    inbound: Node<T>,
    weight: i32,
}

pub struct Graph<T> {
    graph: HashMap<Node<T>, Vec<Edge<T>>>,
}

// `&self` means the method takes an immutable reference.
// If you need a mutable reference, change it to `&mut self` instead.
//
// Your Graph object will be instantiated and called as such:
// let obj = Graph::new(n, edges);
// obj.add_edge(edge);
// let ret_2: i32 = obj.shortest_path(node1, node2);
impl<T: PartialEq + Eq + Hash + Copy> Graph<T> {
    pub fn new(_n: i32, edges: Vec<(T, T, i32)>) -> Self {
        let mut g: HashMap<Node<T>, Vec<Edge<T>>> = HashMap::new();

        for e in edges {
            let edg = Edge {
                outbound: Node { value: e.0 },
                inbound: Node { value: e.1 },
                weight: e.2,
            };
            g.entry(edg.outbound).or_default().push(edg);
        }

        Graph { graph: g }
    }

    pub fn add_edge(&mut self, edge: (T, T, i32)) {
        let edg = Edge {
            outbound: Node { value: edge.0 },
            inbound: Node { value: edge.1 },
            weight: edge.2,
        };

        self.graph.entry(edg.outbound).or_default().push(edg);
    }

    pub fn shortest_path(&self, _node1: i32, _node2: i32) -> i32 {
        0
    }
}
