use std::collections::HashMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = nums1
            .iter()
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x[0]).or_insert(0) += x[1];
                acc
            });

        for n in &nums2 {
            *map.entry(n[0]).or_insert(0) += n[1];
        }

        let mut res: Vec<Vec<i32>> = map
            .into_iter()
            .map(|(k, v)| vec![k, v])
            .collect();
        res.sort_unstable_by_key(|v| v[0]);
        res
    }
}
