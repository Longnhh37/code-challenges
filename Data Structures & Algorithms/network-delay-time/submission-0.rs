use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // prep
        const INF: i32 = i32::MAX;
        let (n, k) = (n as usize, k as usize);

        // adjacency list
        let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];
        for time in &times {
            let (u, v, w) = (time[0], time[1], time[2]);
            adj[u as usize].push((v as usize, w));
        }

        // shortest dist so far
        let mut dist: Vec<i32> = vec![INF; n + 1];
        dist[k] = 0;

        // min-heap to take the shortest path
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, k)));

        // visit mark
        let mut visited = vec![false; n + 1];

        // loop
        while let Some(Reverse((d, u))) = heap.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;

            for &(v, w) in &adj[u] {
                let new_dist = d + w;
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    heap.push(Reverse((new_dist, v)));
                }
            }
        }

        // take result
        let max_dist = dist[1..=n].iter().max().copied().unwrap_or(INF);
        if max_dist == INF {
            -1
        } else {
            max_dist
        }
    }
}
