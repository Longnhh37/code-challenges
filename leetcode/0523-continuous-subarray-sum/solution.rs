use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let k = k as i64;
        let mut seen = HashMap::new();
        seen.insert(0, -1);
        let mut pref_sum = 0;

        for (i, &n) in nums.iter().enumerate() {
            let n = n as i64;
            pref_sum = ((pref_sum + n) % k + k) % k;
            if let Some(&first_idx) = seen.get(&pref_sum) {
                if i as i64 - first_idx >= 2 {
                    return true;
                }
            } else {
                seen.insert(pref_sum, i as i64);
            }
        }
        
        false
    }
}
