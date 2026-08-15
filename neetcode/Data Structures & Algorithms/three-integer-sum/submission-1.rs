use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = HashSet::new();

        for i in 0..n - 1 {
            if nums[i] > 0 {
                break;
            }
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let total = nums[i] + nums[l] + nums[r];
                if total < 0 {
                    l += 1;
                } else if total > 0 {
                    r -= 1;
                } else {
                    res.insert(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                }
            }
        }

        res.into_iter().collect()
    }
}
