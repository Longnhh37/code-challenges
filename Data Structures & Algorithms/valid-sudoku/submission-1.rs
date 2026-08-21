use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen = HashSet::with_capacity(9);

        // row check
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                if !seen.insert(c) {
                    return false;
                }
            }
            seen.clear();
        }

        // col check
        for j in 0..9 {
            for i in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                if !seen.insert(c) {
                    return false;
                }
            }
            seen.clear();
        }

        // box check
        for i in [0, 3, 6] {
            for j in [0, 3, 6] {
                for k in 0..3 {
                    for l in 0..3 {
                        let c = board[i+k][j+l];
                        if c == '.' {
                            continue;
                        }
                        if !seen.insert(c) {
                            return false;
                        }
                    }
                }
                seen.clear();
            }
        }

        true
    }
}
