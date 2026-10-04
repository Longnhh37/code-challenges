use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            adj[e[0] as usize].push((e[1] as usize, e[2] as i64, e[3] as i64));
        }

        let mut dist = vec![i64::MAX; n];
        let mut heap = BinaryHeap::new();
        dist[0] = 0;
        heap.push(Reverse((0i64, 0usize)));

        while let Some(Reverse((t, u))) = heap.pop() {
            if t > dist[u] {
                continue;
            }
            if u == n - 1 {
                return t as i32;
            }
            for &(v, start, end) in &adj[u] {
                if t > end {
                    continue;
                }
                let nt = t.max(start) + 1;
                if nt < dist[v] {
                    dist[v] = nt;
                    heap.push(Reverse((nt, v)));
                }
            }
        }
        
        -1
    }
}
