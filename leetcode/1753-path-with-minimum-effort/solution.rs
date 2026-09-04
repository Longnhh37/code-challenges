use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
const INF: i32 = i32::MAX / 2;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (heights.len(), heights[0].len());
        let (irows, icols) = (rows as i32, cols as i32);

        let mut dist = vec![vec![INF; cols]; rows];
        let mut heap = BinaryHeap::new();

        dist[0][0] = 0;
        heap.push(Reverse((0i32, 0usize, 0usize)));

        while let Some(Reverse((d, r, c))) = heap.pop() {
            if d > dist[r][c] {
                continue;
            }
            if r == rows - 1 && c == cols - 1 {
                return d;
            }

            for (dr, dc) in DIRS {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                    let (ur, uc) = (nr as usize, nc as usize);
                    let edge_w = (heights[ur][uc] - heights[r][c]).abs();
                    let n_effort = d.max(edge_w);
                    if n_effort < dist[ur][uc] {
                        dist[ur][uc] = n_effort;
                        heap.push(Reverse((n_effort, ur, uc)));
                    }
                }
            }
        }

        dist[rows - 1][cols - 1]
    }
}
