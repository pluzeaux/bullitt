use std::collections::HashMap;

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for pair in adjacent_pairs {
        let x = pair[0];
        let y = pair[1];
        graph.entry(x).or_default().push(y);
        graph.entry(y).or_default().push(x);
    }

    let mut array: Vec<i32> = vec![];

    let mut prev = graph.iter().find(|x| x.1.len() == 1).unwrap().0;
    let mut curr = prev;

    loop {
        array.push(*curr);
        let next_head = graph.get(curr).unwrap().iter().find(|&x| x != prev);

        prev = curr;

        match next_head {
            Some(x) => {
                curr = x;
            }
            None => {
                break;
            }
        }
    }

    array
}
