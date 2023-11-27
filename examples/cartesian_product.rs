use std::time::{Duration, Instant};

use bullitt::cartesian_product::cartesian_product;

fn main() {
    let start: Instant = Instant::now();

    const SIZE: i32 = 10;
    let len: i32 = 10;

    let mut set1: Vec<Vec<i32>> = (1..=SIZE).into_iter().map(|i| vec![i]).collect();
    let set2: Vec<i32> = (1..=SIZE).collect();
    (1..len)
        .into_iter()
        .for_each(|_| set1 = cartesian_product(&mut set1, &set2));

    let duration: Duration = start.elapsed();
    println!("size: {}", set1.len());
    println!("{:?}", set1);
    println!(
        "Time elapsed in cartesian_product function is: {:?}",
        duration
    );
}
