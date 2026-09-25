const DIRS: [(i32, i32); 4]  = [(1, 0), (0, 1), (0, -1), (-1, 0)];

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (rows, cols) = (board.len(), board[0].len());

        for r in 0..rows {
            for c in 0..cols {
                if (r == 0 || r == rows - 1 || c == 0 || c == cols - 1)
                    && board[r][c] == 'O' {
                    Self::dfs(board, r, c, rows as i32, cols as i32);
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                } else if board[r][c] == 'S' {
                    board[r][c] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut [Vec<char>], r: usize, c: usize, rows: i32, cols: i32) {
        board[r][c] = 'S';
        let (r, c) = (r as i32, c as i32);

        for (dr, dc) in DIRS {
            let (nr, nc) = (r + dr, c + dc);
            if 0 <= nr && nr < rows && 0 <= nc && nc < cols {
                let (ur, uc) = (nr as usize, nc as usize);
                if board[ur][uc] == 'O' {
                    Self::dfs(board, ur, uc, rows, cols);
                }
            }
        }
    }
}
