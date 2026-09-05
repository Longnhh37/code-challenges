use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut last_idx: HashMap<i32, usize> = HashMap::new();
        let k = k as usize;

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = last_idx.get(&num) {
                if i - j <= k {
                    return true;
                }
            }
            last_idx.insert(num, i);
        }
        false
    }
}
