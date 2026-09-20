use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        Self::backtrack(&nums, 0, &mut Vec::new(), &mut res);
        res.into_iter().collect()
    }

    fn backtrack(
        nums: &[i32],
        start: usize, 
        path: &mut Vec<i32>, 
        res: &mut HashSet<Vec<i32>>
    ) {
        if path.len() >= 2 {
            res.insert(path.clone());
        }

        for i in start..nums.len() {
            if let Some(last) = path.last() && *last > nums[i] {
                continue;
            }
            path.push(nums[i]);
            Self::backtrack(nums, i + 1, path, res);
            path.pop();
        }
    }
}
