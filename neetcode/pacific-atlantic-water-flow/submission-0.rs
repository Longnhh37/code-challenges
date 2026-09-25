const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn pacific_atlantic(mut heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (heights.len(), heights[0].len());

        let mut pacific = vec![vec![false; cols]; rows];
        let mut atlantic = vec![vec![false; cols]; rows];

        for j in 0..cols {
            Self::dfs(&heights, &mut pacific, 0, j);
        }
        for i in 0..rows {
            Self::dfs(&heights, &mut pacific, i, 0);
        }

        for j in 0..cols {
            Self::dfs(&heights, &mut atlantic, rows - 1, j);
        }
        for i in 0..rows {
            Self::dfs(&heights, &mut atlantic, i, cols - 1);
        }

        let mut res = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                if pacific[i][j] && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }

        res
    }

    fn dfs(heights: &[Vec<i32>], visited: &mut [Vec<bool>], i: usize, j: usize) {
        if visited[i][j] { return; }
        visited[i][j] = true;

        let rows = heights.len() as i32;
        let cols = heights[0].len() as i32;

        for (di, dj) in DIRS {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);

            if !visited[ni][nj] && heights[ni][nj] >= heights[i][j] {
                Self::dfs(heights, visited, ni, nj);
            }
        }
    }
}
