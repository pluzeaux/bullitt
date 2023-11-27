pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for (i, v) in nums.iter().enumerate() {
        let diff = &(target - v);

        if i < nums.len() - 1 {
            if nums[..i].contains(diff) || nums[i + 1..].contains(diff) {
                res.push(i as i32)
            }
        } else if nums[..i].contains(diff) {
            res.push(i as i32)
        }
        if res.len() == 2 {
            return res;
        }
    }

    res
}
