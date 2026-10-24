const NONE: usize = usize::MAX;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());

        let mut id = vec![vec![NONE; cols]; rows];
        let (mut k, mut start, mut end) = (0, 0, 0);
        for i in 0..rows {
            for j in 0..cols {
                match grid[i][j] {
                    -1 => continue,
                    1 => start = k,
                    2 => end = k,
                    _ => {}
                }
                id[i][j] = k;
                k += 1;
            }
        }

        let mut adj = vec![Vec::new(); k];
        for i in 0..rows {
            for j in 0..cols {
                let u = id[i][j];
                if u == NONE {
                    continue;
                }
                for (di, dj) in DIRS {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);
                    if ni < 0 || ni >= rows as i32 || nj < 0 || nj >= cols as i32 {
                        continue;
                    }
                    let v = id[ni as usize][nj as usize];
                    if v != NONE {
                        adj[u].push(v);
                    }
                }
            }
        }

        let full = (1usize << k) - 1;
        let mut dp = vec![vec![0u32; k]; 1 << k];
        dp[1 << start][start] = 1;

        for mask in 1..=full {
            for u in 0..k {
                let cnt = dp[mask][u];
                if cnt == 0 {
                    continue;
                }
                for &v in &adj[u] {
                    if mask >> v & 1 == 0 {
                        dp[mask | 1 << v][v] += cnt;
                    }
                }
            }
        }

        dp[full][end] as i32
    }
}
