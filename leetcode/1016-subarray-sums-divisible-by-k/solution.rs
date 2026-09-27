use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut pref_sum = 0;
        let mut map = HashMap::new();       
        map.insert(0, 1);

        for &n in &nums {
            pref_sum = ((pref_sum + n) % k + k) % k;
            count += *map.get(&pref_sum).unwrap_or(&0);
            *map.entry(pref_sum).or_insert(0) += 1;
        }

        count
    }
}
