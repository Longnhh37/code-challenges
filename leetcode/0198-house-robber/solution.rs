impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev2, mut prev) = (0i32, 0i32);

        for &n in &nums {
            let cur = prev.max(prev2 + n);
            (prev2, prev) = (prev, cur);
        }
        prev
    }
}
