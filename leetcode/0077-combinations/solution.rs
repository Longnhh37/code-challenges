impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(1, n, k, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(
        start: i32,
        n: i32,
        k: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if path.len() as i32 == k {
            return res.push(path.clone());
        }
        let remain_needed = k - path.len() as i32;
        for i in start..=(n - remain_needed + 1) {
            path.push(i);
            Self::backtrack(i + 1, n, k, path, res);
            path.pop();
        }
    }
}
