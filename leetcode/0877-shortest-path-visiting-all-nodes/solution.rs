use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let full = (1usize << n) - 1;

        let mut visited = vec![vec![false; n]; 1 << n];
        let mut q = VecDeque::new();

        for i in 0..n {
            let mask = 1usize << i;
            visited[mask][i] = true;
            q.push_back((mask, i, 0));
        }

        while let Some((mask, u, dist)) = q.pop_front() {
            if mask == full {
                return dist;
            }
            for &v in &graph[u] {
                let v = v as usize;
                let next_mask = mask | (1 << v);
                if !visited[next_mask][v] {
                    visited[next_mask][v] = true;
                    q.push_back((next_mask, v, dist + 1));
                }
            }
        }

        0
    }
}
