impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = usize::MAX;
        let mut left = 0;
        let mut sum = 0;

        for (right, &n) in nums.iter().enumerate() {
            sum += n;
            if sum >= target {
                while sum - nums[left] >= target {
                    sum -= nums[left];
                    left += 1;
                }
                res = res.min(right - left + 1);
            }
        }

        if res == usize::MAX { 0 } else { res as i32 }
    }
}
