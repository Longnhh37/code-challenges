const DIRS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut flips = Vec::new();
        let (rows, cols) = (board.len(), board[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        for r in 0..rows {
            for c in 0..cols {
                let cnt = Self::count_living(&board, irows, icols, r as i32, c as i32);

                if (board[r][c] == 1 && (cnt < 2 || cnt > 3))
                    || (board[r][c] == 0 && cnt == 3) {
                        flips.push((r, c));
                }
            }
        }

        for (r, c) in flips {
            board[r][c] = 1 - board[r][c];
        }
    }

    fn count_living(board: &[Vec<i32>],  rows: i32, cols: i32, r: i32, c: i32) -> i32 {
        let mut res = 0;
        for (dr, dc) in DIRS {
            let (nr, nc) = (r + dr, c + dc);
            if 0 <= nr && nr < rows && 0 <= nc && nc < cols {
                let (ur, uc) = (nr as usize, nc as usize);
                if board[ur][uc] == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}
