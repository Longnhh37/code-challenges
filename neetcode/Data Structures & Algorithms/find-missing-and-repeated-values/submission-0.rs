use std::collections::HashSet;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut real_sum = 0;
        let mut extra = 0;
        let mut seen = HashSet::new();
        let n = grid.len();

        for row in &grid {
            for &n in row {
                real_sum += n;
                if !seen.insert(n) {
                    extra = n;
                }
            }
        }


        let theo_sum = (((n * n) + 1) * n * n / 2) as i32;
        let missing = if real_sum > theo_sum {
            extra - (real_sum - theo_sum)
        } else {
            extra + (theo_sum - real_sum)
        };

        vec![extra, missing]
    }
}
