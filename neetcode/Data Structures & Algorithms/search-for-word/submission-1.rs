impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let (rows, cols) = (board.len(), board[0].len());

        for r in 0..rows {
            for c in 0..cols {
                if Self::dfs(&mut board, &word, r as i32, c as i32, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], r: i32, c: i32, i: usize) -> bool {
        if i == word.len() {
            return true;
        }

        if r < 0 || c < 0 || r >= board.len() as i32 || c >= board[0].len() as i32 {
            return false;
        }

        let (ru, rc) = (r as usize, c as usize);
        if board[ru][rc] != word[i] {
            return false;
        }

        board[ru][rc] = '#';
        let res = Self::dfs(board, word, r + 1, c, i + 1)
            || Self::dfs(board, word, r, c + 1, i + 1)
            || Self::dfs(board, word, r - 1, c, i + 1)
            || Self::dfs(board, word, r, c - 1, i + 1);

        board[ru][rc] = word[i];
        res
    }
}
