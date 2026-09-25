use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let s1: HashSet<i32> = nums1.into_iter().collect();
        let s2: HashSet<i32> = nums2.into_iter().collect();

        let res1: Vec<i32> = s1.difference(&s2).cloned().collect();
        let res2: Vec<i32> = s2.difference(&s1).cloned().collect();

        vec![res1, res2]
    }
}
