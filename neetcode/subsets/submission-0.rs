impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(&nums, 0, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(nums: &[i32], start: usize, candidate: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(candidate.clone());

        for i in start..nums.len() {
            candidate.push(nums[i]);
            Self::backtrack(nums, i + 1, candidate, res);
            candidate.pop();
        }
    }
}
