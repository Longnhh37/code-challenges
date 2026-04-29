use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort();
        nums.dedup();
        nums.len() as i32
    }
}
