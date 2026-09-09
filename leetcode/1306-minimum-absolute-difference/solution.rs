impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min_diff = i32::MAX;
        for w in arr.windows(2) {
            min_diff = min_diff.min(w[1] - w[0]);
        }

        let mut res = Vec::new();
        for w in arr.windows(2) {
            if w[1] - w[0] == min_diff {
                res.push(vec![w[0], w[1]]);
            }
        }
        res
    }
}
