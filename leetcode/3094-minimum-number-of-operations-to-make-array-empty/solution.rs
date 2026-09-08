use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let map = nums
            .iter()
            .fold(HashMap::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });       
        let mut res = 0;
        for &v in map.values() {
            if v == 1 {
                return -1;
            } else {
                res += if v % 3 == 0 { v / 3 } else { v / 3 + 1 };
            }
        }
        res
    }
}
