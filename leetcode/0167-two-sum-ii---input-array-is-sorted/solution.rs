impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len() - 1;

        loop {
            let left = nums[l];
            let right = nums[r];

            if left + right < target {
                l += 1;
            } else if left + right > target {
                r -= 1;
            } else {
                return vec![l as i32 + 1, r as i32 + 1];
            }
        }
    }
}
