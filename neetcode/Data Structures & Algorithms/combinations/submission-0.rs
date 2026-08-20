impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(1, n, k as usize, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(start: i32, end: i32, pick: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == pick {
            return res.push(path.clone());
        }

        for i in start..=end {
            path.push(i);
            Self::backtrack(i + 1, end, pick, path, res);
            path.pop();
        }
    }
}
