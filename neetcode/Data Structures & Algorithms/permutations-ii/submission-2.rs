impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut used = vec![false; n];
        let mut path = Vec::new();
        let mut res = Vec::new();
        Self::backtrack(&nums, &mut used, &mut path, &mut res);
        res
    }

    fn backtrack(nums: &[i32], used: &mut Vec<bool>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;
            path.push(nums[i]);
            Self::backtrack(nums, used, path, res);
            path.pop();
            used[i] = false;
        }
    }
}
