impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut res = Vec::new();
        let mut pos = vec![0usize; n];
        Self::dfs(0, n, 0, 0, 0, &mut pos, &mut res);
        res
    }

    fn dfs(
        row: usize,
        n: usize,
        cols: u32,
        diag: u32,
        anti_diag: u32,
        pos: &mut Vec<usize>,
        res: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            res.push(Self::build_board(pos, n));
            return;
        }

        let full_mask = (1u32 << n) - 1;
        let mut avail = full_mask & !(cols | diag | anti_diag);

        while avail != 0 {
            let bit = avail & avail.wrapping_neg();
            avail -= bit;
            pos[row] = bit.trailing_zeros() as usize;


            Self::dfs(row + 1, n, cols | bit, (diag | bit) << 1, (anti_diag | bit) >> 1, pos, res);
        }
    }

    fn build_board(pos: &[usize], n: usize) -> Vec<String> {
        pos
        .iter()
        .map(|&c| {
            let mut row = vec![b'.'; n];
            row[c] = b'Q';
            String::from_utf8(row).unwrap()
        })
        .collect()
    }
}
