use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0usize, 0usize)));

        const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        while let Some(Reverse((t, r, c))) = heap.pop() {
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;

            if r == n - 1 && c == n - 1 {
                return t;
            }

            for (dr, dc) in DIRS {
                let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) 
                else {
                    continue;
                };
                if nr < n && nc < n && !visited[nr][nc] {
                    heap.push(Reverse((t.max(grid[nr][nc]), nr, nc)));
                }
            }
        }

        unreachable!()
    }
}
