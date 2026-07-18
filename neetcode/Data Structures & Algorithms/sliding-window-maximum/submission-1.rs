use std::collections::BTreeMap;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut map = BTreeMap::new();
        let mut res = Vec::with_capacity(n - k + 1);

        for i in 0..n {
            *map.entry(nums[i]).or_insert(0) += 1;

            if i >= k - 1 {
                if let Some(&max) = map.keys().last() {
                    res.push(max);
                }

                if let Some(count) = map.get_mut(&nums[i - k + 1]) {
                    *count -= 1;
                    if *count == 0 {
                        map.remove(&nums[i - k + 1]);
                    }
                }
            }
        }

        res
    }
}
