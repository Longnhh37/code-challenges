impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut cur_sum = 0;
        for &n in &nums {
            if cur_sum < 0 {
                cur_sum = 0;
            }
            cur_sum += n;
            res = res.max(cur_sum);
        }
        res
    }
}
