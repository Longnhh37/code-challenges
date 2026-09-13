const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let (rows, cols) = (board.len(), board[0].len());

        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == chars[0] {
                    board[r][c] = '#';
                    if Self::backtrack(&mut board, &chars, 1, r, c) {
                        return true;
                    }
                    board[r][c] = chars[0];
                }
            }
        }
        false
    }

    fn backtrack(
        board: &mut [Vec<char>], 
        chars: &[char], 
        next_char: usize, 
        r: usize, 
        c: usize
    ) -> bool {
        if next_char == chars.len() {
            return true;
        }

        let (rows, cols) = (board.len(), board[0].len());
        for (dr, dc) in DIRS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if board[ur][uc] == chars[next_char] {
                board[ur][uc] = '#';
                if Self::backtrack(board, chars, next_char + 1, ur, uc) {
                    return true;
                }
                board[ur][uc] = chars[next_char];
            } 
        }
        false
    }
}
