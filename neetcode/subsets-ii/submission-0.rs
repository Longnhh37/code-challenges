use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        Self::backtrack(&nums, 0, &mut Vec::new(), &mut res);
        res
            .into_iter()
            .collect::<Vec<Vec<i32>>>()
    }

    fn backtrack(nums: &[i32], start: usize, candidate: &mut Vec<i32>, res: &mut HashSet<Vec<i32>>) {
        let mut candidate_cp = candidate.clone();
        candidate_cp.sort_unstable();
        res.insert(candidate_cp);

        for i in start..nums.len() {
            candidate.push(nums[i]);
            Self::backtrack(nums, i + 1, candidate, res);
            candidate.pop();
        }
    }
}