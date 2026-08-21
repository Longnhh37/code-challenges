impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        } else if n == 2 {
            return nums[0].max(nums[1]);
        }
        
        let include_first = Self::most_rob(&nums, 0, n - 2);
        let exclude_first = Self::most_rob(&nums, 1, n - 1);
        include_first.max(exclude_first)
    }

    fn most_rob(nums: &[i32], start: usize, end: usize) -> i32 {
        let (mut prev_2, mut prev) = (0i32, 0i32);
        for i in start..=end {
            let n = nums[i];
            let cur = prev.max(prev_2 + n);
            (prev_2, prev) = (prev, cur);
        }
        prev
    }
}
