use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums
            .into_iter()
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
            .values() 
            .try_fold(0, |acc, &v| {
                if v == 1 { None } else { Some(acc + (v + 2) / 3) }
            }).unwrap_or(-1) 
    }
}
