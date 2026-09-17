impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = Vec::new();
        let mut visited = vec![false; nums.len()];
        Self::backtrack(&nums, &mut visited, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(nums: &[i32], visited: &mut [bool], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            return res.push(path.clone());
        }
        for (i, &n) in nums.iter().enumerate() {
            if visited[i] {
                continue;
            }
            if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
                continue;
            }
            visited[i] = true;
            path.push(n);
            Self::backtrack(nums, visited, path, res);
            path.pop();
            visited[i] = false;
        }
    }
}
