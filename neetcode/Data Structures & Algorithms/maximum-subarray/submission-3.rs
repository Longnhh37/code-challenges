impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        let mut sum = nums[0];

        for &cur in nums.iter().skip(1) {
            prev = cur.max(cur + prev);
            sum = sum.max(prev);
        }

        sum
    }
}
