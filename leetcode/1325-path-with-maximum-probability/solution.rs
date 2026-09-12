use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
struct Prob(f64);

impl Eq for Prob {}
impl Ord for Prob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Prob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let (n, src, dest) = (n as usize, start_node as usize, end_node as usize);
        let mut adj = vec![Vec::new(); n];

        for (e, &w) in edges.iter().zip(succ_prob.iter()) {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        let mut dist = vec![0.0f64; n];
        let mut visited = vec![false; n];
        let mut heap = BinaryHeap::new();
        dist[src] = 1.0;
        heap.push((Prob(1.0), src));

        while let Some((Prob(cur_prob), u)) = heap.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for &(v, next_prob) in &adj[u] {
                let new_prob = cur_prob * next_prob;
                if new_prob > dist[v] {
                    dist[v] = new_prob;
                    heap.push((Prob(new_prob), v))
                }
            }
        }

        dist[dest]
    }
}
