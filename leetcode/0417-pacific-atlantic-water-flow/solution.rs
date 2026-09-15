const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

const PACIFIC: u8 = 0b01;
const ATLANTIC: u8 = 0b10;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (heights.len(), heights[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        let mut reach = vec![0u8; rows * cols];

        for r in 0..rows {
            Self::dfs(&heights, &mut reach, cols, r, 0, PACIFIC);
            Self::dfs(&heights, &mut reach, cols, r, cols - 1, ATLANTIC);
        }
        for c in 0..cols {
            Self::dfs(&heights, &mut reach, cols, 0, c, PACIFIC);
            Self::dfs(&heights, &mut reach, cols, rows - 1, c, ATLANTIC);
        }

        let mut res = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if reach[r * cols + c] == PACIFIC | ATLANTIC {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }

        res
    }
    
    fn dfs(
        heights: &[Vec<i32>], reach: &mut [u8],
        cols: usize,
        r: usize, c: usize,
        flag: u8,
    ) {
        if reach[r * cols + c] & flag != 0 {
            return;
        }
        reach[r * cols + c] |= flag;

        let rows = heights.len();
        let irows = rows as i32;
        let icols = cols as i32;

        for (dr, dc) in DIRS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nr >= irows || nc < 0 || nc >= icols {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if heights[ur][uc] >= heights[r][c] {
                Self::dfs(heights, reach, cols, ur, uc, flag);
            }
        }
    }
}
