use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums2_idx = HashMap::new();
        for (i, &x) in nums2.iter().enumerate() {
            nums2_idx.insert(x, i);
        }

        let mut next_greater= vec![None; nums2.len()];
        let mut stack = Vec::new();

        for (i, &n) in nums2.iter().enumerate().rev() {
            while let Some(&top) = stack.last() {
                if top <= n {
                    stack.pop();
                } else {
                    break;
                }
            }
            next_greater[i] = stack.last().copied();
            stack.push(n);
        }

        let mut res = Vec::with_capacity(nums1.len());
        for n in &nums1 {
            let i = *nums2_idx.get(n).unwrap();
            res.push(next_greater[i].unwrap_or(-1));
        }

        res
    }
}
