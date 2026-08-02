impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums[0]
            .max(Self::helper(&nums[1..]))
            .max(Self::helper(&nums[..nums.len() - 1]))
    }

    fn helper(nums: &[i32]) -> i32 {
        let (mut prev_2, mut prev) = (0i32, 0i32);
        for &n in nums {
            let cur = prev.max(prev_2 + n);
            (prev_2, prev) = (prev, cur);
        }
        prev
    }
}
