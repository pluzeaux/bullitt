use std::time::{Duration, Instant};

use bullitt::two_sum::two_sum;

fn main() {
    let nums = [-3, 4, 3, 90].iter().cloned().collect::<Vec<i32>>();
    let target = 0;
    let start: Instant = Instant::now();
    let ts = two_sum(nums, target);
    let duration: Duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    println!("{:?}", ts)
}
