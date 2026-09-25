impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev_2, mut prev) = (0_i32, 0_i32);

        for &n in &nums {
            let cur = prev.max(prev_2 + n);
            (prev_2, prev) = (prev, cur);
        }

        prev
    }
}
