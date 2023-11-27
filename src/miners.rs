use std::collections::HashMap;

pub fn create_map(hm: HashMap<(i32, i32), char>) -> HashMap<(i32, i32), char> {
    hm.iter()
        //    .sorted_unstable_by_key(|x| x.0)
        .map(|(k, v)| {
            let re: ((i32, i32), char) = if v == &' ' {
                let mut nb_mines: u32 = 0;
                nb_mines += is_mine(&(k.0 - 1, k.1 - 1), &hm);
                nb_mines += is_mine(&(k.0 - 1, k.1), &hm);
                nb_mines += is_mine(&(k.0 - 1, k.1 + 1), &hm);
                nb_mines += is_mine(&(k.0, k.1 - 1), &hm);
                nb_mines += is_mine(&(k.0, k.1 + 1), &hm);
                nb_mines += is_mine(&(k.0 + 1, k.1 - 1), &hm);
                nb_mines += is_mine(&(k.0 + 1, k.1), &hm);
                nb_mines += is_mine(&(k.0 + 1, k.1 + 1), &hm);

                (*k, char::from_digit(nb_mines, 10).unwrap())
            } else {
                (*k, *v)
            };
            re
        })
        .collect()
}

pub fn populate_vec(hm: HashMap<(i32, i32), char>, shape: (u32, u32)) -> Vec<String> {
    (0..shape.0)
        .map(|i| {
            (0..shape.1)
                .map(|j| hm.get(&(i as i32, j as i32)).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

pub fn isolate_char(ss: &[&str]) -> HashMap<(i32, i32), char> {
    ss.iter()
        .enumerate()
        .flat_map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect::<HashMap<(i32, i32), char>>()
}

fn is_mine(key: &(i32, i32), hm: &HashMap<(i32, i32), char>) -> u32 {
    if hm.contains_key(key) {
        if let Some(v) = hm.get(key) {
            if v == &'*' {
                return 1_u32;
            } else {
                return 0_u32;
            }
        }
    }
    0
}
