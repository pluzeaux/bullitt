pub fn count_homogenous(s: String) -> i32 {
    let vc: Vec<char> = s.chars().collect();
    let mut vvc: Vec<Vec<char>> = Vec::new();

    let mut ls: Vec<char> = vec![vc[0]];

    for i in 1..vc.len() {
        if vc[i - 1] != vc[i] {
            vvc.push(ls);
            ls = Vec::new();
        }
        ls.push(vc[i]);
    }
    vvc.push(ls);

    let res = vvc
        .iter()
        .map(|s| s.len() as i128 * (s.len() + 1) as i128 / 2)
        .sum::<i128>();

    if res > 10_i128.pow(9) {
        (res % (10_i128.pow(9) + 7)) as i32
    } else {
        res as i32
    }
}
