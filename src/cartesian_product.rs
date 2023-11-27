pub fn cartesian_product(in1: &mut [Vec<i32>], in2: &[i32]) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    in1.iter_mut().for_each(|el1| {
        in2.iter().for_each(|e2| {
            if el1.last().unwrap() < e2 {
                let mut el1c: Vec<i32> = el1.clone();
                el1c.push(*e2);
                res.push(el1c);
            }
        })
    });

    res
}
