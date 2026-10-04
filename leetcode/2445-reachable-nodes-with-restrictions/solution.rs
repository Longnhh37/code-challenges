use std::collections::{VecDeque};

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut visited = vec![false; n];
        for &r in &restricted {
            visited[r as usize] = true;
        }
        visited[0] = true;

        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut q = VecDeque::from([0]);
        let mut count = 0;

        while let Some(u) = q.pop_front() {
            count += 1;
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }

        count
    }
}
