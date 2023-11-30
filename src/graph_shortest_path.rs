use std::{cmp::Ordering, collections::HashMap, hash::Hash};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Node<T> {
    pub value: T,
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

impl<T: PartialEq + Eq + Hash + Copy + Ord> Graph<T> {
    pub fn new(edges: Vec<(T, T, i32)>) -> Self {
        let g: HashMap<Node<T>, Vec<Edge<T>>> = HashMap::new();
        let mut graph= Graph{graph: g};

        for e in edges {
            graph.add_edge(e);
        }

        graph
    }

    pub fn add_edge(&mut self, edge: (T, T, i32)) {
        let edg = Edge {
            outbound: Node { value: edge.0 },
            inbound: Node { value: edge.1 },
            weight: edge.2,
        };

        self.graph.entry(edg.outbound).or_default().push(edg);
    }

    pub fn shortest_path(&self, node1: Node<T>, node2: Node<T>) -> i32 {
        let mut stack: Vec<Path<T>> = Vec::new();
        stack.push(Path {
            current: Some(node1),
            weight: 0,
        });
        let mut res: Vec<Path<T>> = Vec::new();

        while let Some(path) = stack.pop() {
            if let Some(neighbours) = self.graph.get(&path.current.unwrap()) {
                for n in neighbours {
                    if n.inbound != node2 {
                        stack.push(Path {
                            current: Some(n.inbound),
                            weight: path.weight + n.weight,
                        })
                    } else {
                        res.push(Path {
                            current: Some(n.inbound),
                            weight: path.weight + n.weight,
                        })
                    }
                }
            };
        }

        let r = res.iter().min().unwrap();
        r.weight
    }
}

#[derive(Eq, Clone)]
struct Path<T: Ord> {
    current: Option<Node<T>>,
    weight: i32,
}

impl<T: Ord> Ord for Path<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: Ord> PartialOrd for Path<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> PartialEq for Path<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
