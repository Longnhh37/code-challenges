use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
struct FloatOrd(f64);
impl Eq for FloatOrd {}
impl PartialOrd for FloatOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
        for (i, edge) in edges.iter().enumerate() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            let p = succ_prob[i];
            adj[a].push((b, p));
            adj[b].push((a, p));
        }

        let mut dist = vec![0.0f64; n];
        let start = start_node as usize;
        let end = end_node as usize;
        dist[start] = 1.0;

        let mut heap = BinaryHeap::new();
        heap.push((FloatOrd(1.0), start));

        while let Some((FloatOrd(prob), cur)) = heap.pop() {
            if cur == end {
                return prob;
            }
            if prob < dist[cur] {
                continue;
            }
            for &(next, w) in &adj[cur] {
                let new_prob = prob * w;
                if new_prob > dist[next] {
                    dist[next] = new_prob;
                    heap.push((FloatOrd(new_prob), next));
                }
            }
        }
        
        dist[end]
    }
}
