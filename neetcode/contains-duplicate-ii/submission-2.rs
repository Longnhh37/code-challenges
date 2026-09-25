use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }

        let mut map = HashMap::new();

        for (cur, n) in nums.iter().enumerate() {
            if let Some(last) = map.get(n) && cur - last <= k as usize {
                return true;
            } else {
                map.insert(n, cur);
            }
        }

        false
    }
}
