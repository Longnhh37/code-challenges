use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k <= 0 || nums.len() < 2 {
            return false;
        }
        let k = k as usize;
        let mut window = HashSet::with_capacity(k + 1);

        for (i, &n) in nums.iter().enumerate() {
            if i > k { 
                window.remove(&nums[i - k - 1]);
            }
            if !window.insert(n) {
                return true;
            }
        }

        false
    }
}
