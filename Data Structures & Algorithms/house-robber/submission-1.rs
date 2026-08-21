impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        }
        let mut prev_prev = nums[0];
        let mut prev  = nums[0].max(nums[1]);

        for i in 2..n {
            let cur = prev.max(prev_prev + nums[i]);
            (prev_prev, prev) = (prev, cur);
        }

        prev
    }
}
