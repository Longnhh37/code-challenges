impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 1 - nums[0];
        }

        let mut total = ((len + 1) * len / 2) as i32;
        for v in nums {
            total -= v;
        }

        total
    }
}
