impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];

        if n == 0 {
            return res;
        }

        let (mut l, mut r) = (0usize, n - 1);

        for i in (0..n).rev() {
            if nums[l].abs() > nums[r].abs() {
                res[i] = nums[l] * nums[l];
                l += 1;
            } else {
                res[i] = nums[r] * nums[r];
                if r == 0 { break; }
                r -= 1;
            }
        }

        res
    }
}

