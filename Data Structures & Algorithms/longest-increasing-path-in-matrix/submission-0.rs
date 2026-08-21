const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut res = 0_i32;
        let mut memo = vec![vec![0_i32; n]; m];

        for r in 0..m {
            for c in 0..n {
                res = res.max(Self::dfs(r, c, &matrix, &mut memo));
            }
        }
        res
    }

    fn dfs(r: usize, c: usize, matrix: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[r][c] != 0 {
            return memo[r][c];
        }

        let mut max_path = 1_i32;
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);

        for (dr, dc) in DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= m || nc >= n {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if matrix[ur][uc] > matrix[r][c] {
                max_path = max_path.max(Self::dfs(ur, uc, matrix, memo) + 1);
            }
        }
        memo[r][c] = max_path;
        max_path
    }
}
