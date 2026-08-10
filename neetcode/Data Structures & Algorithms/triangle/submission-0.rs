impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut prev = vec![i32::MAX; n];
        prev[0] = triangle[0][0];

        for i in 1..n {
            let mut cur = vec![i32::MAX; n];
            for j in 0..=i {
                cur[j] = triangle[i][j] + prev[j.saturating_sub(1)].min(prev[j]);
            }
            prev = cur;
        }

        *prev.iter().min().unwrap()
    }
}
