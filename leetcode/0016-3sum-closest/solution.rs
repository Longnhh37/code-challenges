impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut best_diff = i32::MAX;

        for i in 0..n {
            let (mut l, mut r) = (i + 1, n - 1);
            let pre_diff = nums[i] - target;
            while l < r {
                let diff = pre_diff + nums[l] + nums[r];
                if diff.abs() < best_diff.abs() {
                    best_diff = diff;
                }
                match diff.signum() {
                    1 => r -= 1,
                    -1 => l += 1,
                    0 => return target,
                    _ => unreachable!(),
                }
            }
        }

        target + best_diff
    }
}
