use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

const MOD: u64 = 1_000_000_007;
const INF: u64 = u64::MAX;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let src: usize = 0;
        let mut adj = vec![Vec::new(); n];
        for r in &roads {
            let (u, v, w) = (r[0] as usize, r[1] as usize, r[2] as u64);
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        let mut dist = vec![INF; n];
        let mut cnt = vec![0u64; n];
        let mut heap = BinaryHeap::new();

        dist[src] = 0;
        cnt[src] = 1;
        heap.push(Reverse((0, src)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }

            for &(v, w) in &adj[u] {
                let nd = d + w;
                match nd.cmp(&dist[v]) {
                    Ordering::Less => {
                        dist[v] = nd;
                        cnt[v] = cnt[u];
                        heap.push(Reverse((nd, v)));
                    }
                    Ordering::Equal => cnt[v] = (cnt[v] + cnt[u]) % MOD,
                    Ordering::Greater => {}
                }
            }
        }

        cnt[n - 1] as i32
    }
}
