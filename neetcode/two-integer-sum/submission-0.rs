use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut seen = HashMap::new();
       for (idx, num) in nums.iter().enumerate() {
            if let Some(first) = seen.get(&(target - num)) {
                return vec![*first, idx as i32];
            }
            seen.insert(num, idx as i32);
       }        
       unreachable!();
    }
}
