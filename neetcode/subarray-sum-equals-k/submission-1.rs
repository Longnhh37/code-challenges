use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        count.insert(0, 1);

        let mut res = 0;
        let mut sum = 0;

        for &n in &nums {
            sum += n;
            res += *count.get(&(sum - k)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        res
    }
}
