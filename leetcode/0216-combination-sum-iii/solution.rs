impl Solution {
    pub fn combination_sum3(k: i32, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(1, k as usize, target, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(
        start: i32,
        k: usize, 
        remaining: i32, 
        path: &mut Vec<i32>, 
        res: &mut Vec<Vec<i32>>
    ) {
        if path.len() == k {
            if remaining == 0 {
                res.push(path.clone());
            }
            return;
        }

        let need = (k - path.len()) as i32;
        for x in start..=(10 - need) {
            if x > remaining {
                break;
            }
            path.push(x);
            Self::backtrack(x + 1, k, remaining - x, path, res);
            path.pop();
        }
    }

}
