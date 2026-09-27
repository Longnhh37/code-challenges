use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (height_map.len(), height_map[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        let mut visited = vec![vec![false; cols]; rows];
        let mut heap = BinaryHeap::new();

        for r in 0..rows {
            for c in [0, cols -1] {
                visited[r][c] = true;
                heap.push(Reverse((height_map[r][c], r, c)));
            }
        }
        for c in 0..cols {
            for r in [0, rows - 1] {
                visited[r][c] = true;
                heap.push(Reverse((height_map[r][c], r, c)));
            }
        }

        let mut res = 0;

        while let Some(Reverse((h, r, c))) = heap.pop() {
            for (dr, dc) in DIRS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                    let ur = nr as usize;
                    let uc = nc as usize;
                    if !visited[ur][uc] {
                        visited[ur][uc] = true;
                        res += (h - height_map[ur][uc]).max(0);
                        heap.push(Reverse((height_map[ur][uc].max(h), ur, uc)));
                    }
                }
            }
        }

        res
    }
}
