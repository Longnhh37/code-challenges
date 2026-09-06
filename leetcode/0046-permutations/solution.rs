use std::collections::HashSet;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(&nums, &mut HashSet::new(), &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(nums: &[i32], seen: &mut HashSet<i32>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            return res.push(path.clone());
        }

        for &n in nums {
            if seen.insert(n) {
                path.push(n);
                Self::backtrack(nums, seen, path, res);
                path.pop();
                seen.remove(&n);
            }
        }
    }
}
