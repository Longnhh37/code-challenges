use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let map = nums
            .into_iter()
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0i32) += 1;
                acc
            });

        let mut res = 0;
        for &v in map.values() {
            if v == 1 {
                return -1;
            }
            let n = v / 3;
            if v % 3 == 0 {
                res += n;
            } else {
                res += n + 1;
            }
        }

        res
    }
}
