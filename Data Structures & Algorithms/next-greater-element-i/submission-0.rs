use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_idx: HashMap<i32, usize> = nums1
            .iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut res = vec![-1i32; nums1.len()];

        for i in 0..nums2.len() {
            if !nums1_idx.contains_key(&nums2[i]) {
                continue;
            }
            for j in i + 1..nums2.len() {
                if nums2[j] > nums2[i] {
                    let idx = nums1_idx[&nums2[i]];
                    res[idx] = nums2[j];
                    break;
                }
            }
        }

        res
    }
}
