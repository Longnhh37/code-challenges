impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut used = vec![false; n];
        let mut res = Vec::new();
        Self::backtrack(&nums, 0, &mut Vec::new(), &mut used, &mut res);
        res
    }

    fn backtrack(nums: &[i32], start: usize, path: &mut Vec<i32>, used: &mut [bool], res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            return res.push(path.clone());
        }
        for i in 0..nums.len() {
            if used[i] { continue; }
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            Self::backtrack(nums, i + 1, path, used, res);
            path.pop();
            used[i] = false;
        }
    }
}
