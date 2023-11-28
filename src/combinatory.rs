pub fn combine<T: Copy>(set: Vec<T>, nb_element: i32, nb_parties: u128) -> Vec<Vec<T>> {
    let ss: Vec<Vec<char>> = (0..nb_parties)
        .map(|i| format!("{:0nb_element$b}", i, nb_element = nb_element as usize))
        .filter(|s| s.matches('1').count() == set.len())
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    ss.iter()
        .map(|s| {
            s.iter()
                .zip(&set)
                .filter(|s| *s.0 != '0')
                .map(|c| *c.1)
                .collect::<Vec<T>>()
        })
        .collect()
}
