use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut pref_sum = 0;
        let mut seen = HashMap::new();
        seen.insert(0, 1);

        for &n in &nums {
            pref_sum += n;
            count += *seen.get(&(pref_sum - k)).unwrap_or(&0);
            *seen.entry(pref_sum).or_insert(0) += 1;
        }

        count
    }
}
