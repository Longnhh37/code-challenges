impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for &n in &nums {
            res |= n;
        }
        res << (nums.len() - 1)
    }
}
