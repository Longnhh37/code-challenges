impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        let (mut l, mut r) = (0, n - 1);
        let mut w_idx = n - 1;

        while l <= r {
            let left = nums[l] * nums[l];
            let right = nums[r] * nums[r];

            if left < right {
                res[w_idx] = right;
                r -= 1;
            } else {
                res[w_idx] = left;
                l += 1;
            }
            w_idx -= 1;
        }
        res
    }
}
