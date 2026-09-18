impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
       let n = n as usize;
       let mut cols = vec![false; n];
       let mut diag1 = vec![false; 2 * n]; // row + n - col
       let mut diag2 = vec![false; 2 * n]; // row + col
       let mut queens = vec![0usize; n]; // queens[row] = col
       let mut res = Vec::new();
       Self::backtrack(0, n, &mut cols, &mut diag1, &mut diag2, &mut queens, &mut res);
       res
    }

    fn backtrack(
        row: usize,
        n: usize,
        cols: &mut [bool],
        diag1: &mut [bool],
        diag2: &mut [bool],
        queens: &mut [usize],
        res: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            return res.push(Self::build_board(&queens, n));
        }
        for col in 0..n {
            let d1 = row + n - col;
            let d2 = row + col;
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;
            queens[row] = col;

            Self::backtrack(row + 1, n, cols, diag1, diag2, queens, res);

            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }

    fn build_board(queens: &[usize], n: usize) -> Vec<String> {
        queens
            .iter()
            .map(|&col| {
                let mut cur = vec![b'.'; n];
                cur[col]  = b'Q';
                String::from_utf8(cur).unwrap()
            })
            .collect()
    }
}
