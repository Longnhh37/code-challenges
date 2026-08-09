use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const INF: i32 = i32::MAX;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (heights.len(), heights[0].len());
        let (rows_i, cols_i) = (rows as i32, cols as i32);
        let mut visited = vec![vec![false; cols]; rows];
        let mut min_dst = vec![vec![INF; cols]; rows];
        min_dst[0][0] = 0;

        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0)));

        while let Some(Reverse((d, r, c))) = heap.pop() {
            if r == rows - 1 && c == cols - 1 {
                return d;
            }
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;

            let (ri, ci) = (r as i32, c as i32);
            for (dr, dc) in DIRS {
                let (nr, nc) = (ri + dr, ci + dc);
                if 0 > nr || nr >= rows_i || 0 > nc || nc >= cols_i {
                    continue;
                }
                let (ur, uc) = (nr as usize, nc as usize);
                let edge = (heights[r][c] - heights[ur][uc]).abs();
                let new_dist = d.max(edge);
                if new_dist < min_dst[ur][uc] {
                    min_dst[ur][uc] = new_dist;
                    heap.push(Reverse((new_dist, ur, uc)));
                }
            }
        }

        heights[rows - 1][cols - 1]
    }
}
