use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let store: HashSet<i32> = nums.iter().cloned().collect();

        for &n in &nums {
            let mut streak = 0;
            let mut cur = n;
            while store.contains(&cur) {
                streak += 1;
                cur += 1;
            }
            res = res.max(streak);
        }

        res
    }
}
