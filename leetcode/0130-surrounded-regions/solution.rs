const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (rows, cols) = (board.len(), board[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        for c in 0..cols {
            if board[0][c] == 'O' {
                Self::dfs(board, 0, c, irows, icols);
            }
            if board[rows - 1][c] == 'O' {
                Self::dfs(board, rows - 1, c, irows, icols);
            }
        }

        for r in 0..rows {
            if board[r][0] == 'O' {
                Self::dfs(board, r, 0, irows, icols);
            }
            if board[r][cols - 1] == 'O' {
                Self::dfs(board, r, cols - 1, irows, icols);
            }
        }

        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == '#' {
                    board[r][c] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, irows: i32, icols: i32) {
        board[r][c] = '#';
        for (dr, dc) in DIRS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nr >= irows || nc < 0 || nc >= icols {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if board[ur][uc] == 'O' {
                Self::dfs(board, ur, uc, irows, icols);
            }
        }
    }

    
}
