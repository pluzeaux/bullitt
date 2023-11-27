use bullitt::two_sum::two_sum;
fn main() {
    let nums = [3, 2, 4].iter().cloned().collect::<Vec<i32>>();
    let target = 6;

    let ts = two_sum(nums, target);
    println!("{:?}", ts)
}
