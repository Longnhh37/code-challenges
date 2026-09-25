impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut cur_max = nums[0];
        let mut cur_min = nums[0];
        let mut res = nums[0];

        for &num in nums.iter().skip(1) {
            if num < 0 {
                std::mem::swap(&mut cur_max, &mut cur_min);
            }

            cur_max = num.max(cur_max.saturating_mul(num));
            cur_min = num.min(cur_min.saturating_mul(num));

            res = res.max(cur_max);
        }

        res
    }
}
