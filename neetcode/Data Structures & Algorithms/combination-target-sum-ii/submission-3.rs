impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut res = Vec::new();
        Self::backtrack(&candidates, 0, target, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(
        candidates: &[i32],
        start: usize, 
        remaining: i32, 
        path: &mut Vec<i32>, 
        res: &mut Vec<Vec<i32>>
    ) {
        if remaining == 0 {
            res.push(path.clone());
            return 
        }

        for i in start..candidates.len() {
            if candidates[i] > remaining {
                break;
            }
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            path.push(candidates[i]);
            Self::backtrack(candidates, i + 1, remaining - candidates[i], path, res);
            path.pop();
        }
    }
}
