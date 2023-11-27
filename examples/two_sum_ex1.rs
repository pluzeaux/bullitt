use bullitt::two_sum::two_sum;

fn main() {
    let nums = [2, 7, 11, 15].iter().cloned().collect::<Vec<i32>>();
    let target = 9;

    let ts = two_sum(nums, target);
    println!("{:?}", ts)
}
