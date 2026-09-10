use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut graph = vec![Vec::new(); n + 1];
        for t in times {
            let (u, v, w) = (t[0] as usize, t[1] as usize, t[2]);
            graph[u].push((v, w));
        }

        let mut dist = vec![i32::MAX; n + 1];
        dist[k] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i32, k)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] { continue; }
            for &(v, w) in &graph[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }

        let max_dist = dist[1..=n].iter().max().copied().unwrap_or(i32::MAX);
        if max_dist == i32::MAX { -1 } else { max_dist }

    }
}
