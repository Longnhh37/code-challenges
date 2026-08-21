use std::collections::HashSet;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(nums.len());
        let mut candidate = Vec::with_capacity(nums.len());
        let mut seen = HashSet::new();
        Self::backtrack(&nums, &mut candidate, &mut seen, &mut res);
        res
    }

    fn backtrack(nums: &[i32], candidate: &mut Vec<i32>, seen: &mut HashSet<i32>, res: &mut Vec<Vec<i32>>) {
        if candidate.len() == nums.len() {
            return res.push(candidate.clone());
        }
        for &n in nums {
            if seen.insert(n) {
                candidate.push(n);
                Self::backtrack(nums, candidate, seen, res);
                candidate.pop();
                seen.remove(&n);
            }
        }
    }
}
