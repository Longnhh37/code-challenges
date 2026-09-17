impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut seen = vec![false; nums.len()];
        Self::backtrack(&nums, &mut seen, &mut Vec::new(), &mut res);

        res
    }
    
    fn backtrack(nums: &[i32], seen: &mut [bool], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            return res.push(path.clone());
        }
        
        for (i, &n) in nums.iter().enumerate() {
            if !seen[i] {
                seen[i] = true;
                path.push(n);
                Self::backtrack(nums, seen, path, res);
                path.pop();
                seen[i] = false;
            }
        }
    }
}
