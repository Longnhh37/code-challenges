use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: u64 = u64::MAX / 4;

fn dijkstra(adj: &[Vec<(usize, u64)>], src: usize) -> Vec<u64> {
    let mut dist = vec![INF; adj.len()];
    let mut heap = BinaryHeap::new();

    dist[src] = 0;
    heap.push(Reverse((0u64, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = w + d;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }
    dist
}

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as u64);
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        let d0 = dijkstra(&adj, 0);
        let dn = dijkstra(&adj, n - 1);
        let total = d0[n - 1];

        if total >= INF {
            return vec![false; edges.len()];
        }

        edges
            .iter()
            .map(|e| {
                let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as u64);
                d0[u] + w + dn[v] == total || d0[v] + w + dn[u] == total
            })
            .collect()
    }
}
