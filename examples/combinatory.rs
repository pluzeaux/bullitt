use bullitt::combinatory::combine;
use std::time::{Duration, Instant};

fn main() {
    const SIZE: i32 = 100;
    let set2: Vec<i32> = (1..=SIZE).collect();
    let _len: i32 = 4;

    let set = ['a', 'b', 'c'];
    let nb_element = set2.len();
    let nb_parties = u128::pow(2, nb_element as u32);

    let start: Instant = Instant::now();

    let binding = set.to_vec();
    let res = combine(&binding, nb_element.try_into().unwrap(), nb_parties);

    let duration: Duration = start.elapsed();
    println!(
        "Time elapsed in cartesian_product function is: {:?}",
        duration
    );

    for r in res {
        println!("res: {:?}", r)
    }
}
