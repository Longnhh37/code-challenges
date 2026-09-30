impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let cols = colsum.len();
        let mut res = vec![vec![-1; cols]; 2];

        for i in 0..cols {
            if colsum[i] == 0 {
                res[0][i] = 0;
                res[1][i] = 0;
            } else if colsum[i] == 2 {
                upper -= 1;
                lower -= 1;
                res[0][i] = 1;
                res[1][i] = 1;
            }
        }

        if upper < 0 || lower < 0 {
            return Vec::new();
        }

        let mut i = 0;
        while upper > 0 && i < cols {
            if res[0][i] == -1 {
                res[0][i] = 1;
                res[1][i] = 0;
                upper -= 1;
            }
            i += 1;
        }

        while lower > 0 && i < cols {
            if res[1][i] == -1 {
                res[0][i] = 0;
                res[1][i] = 1;
                lower -= 1;
            }
            i += 1;
        }

        if upper > 0 || lower > 0 || res.iter().any(|row| row.iter().any(|cell| *cell == -1)) {
            Vec::new()
        } else {
            res
        }
    }
}
