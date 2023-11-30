use bullitt::graph_shortest_path::{Graph, Node};

fn main() {
   let graph1 = Graph::new(
    vec![
        (0, 2, 5),
        (0, 1, 2),
        (1, 2, 1),
        (3, 0, 3),
    ]
   );

   let res = graph1.shortest_path(Node{value: 3}, Node{value: 2});
   println!("graph1: {}", res);

   let graph2 = Graph::new(
    vec![
        ('a', 'c', 5),
        ('a', 'b', 2),
        ('b', 'c', 1),
        ('d', 'a', 3),
    ]
   );

   let res = graph2.shortest_path(Node{value: 'd'}, Node{value: 'c'});
   println!("graph2: {}", res);
}