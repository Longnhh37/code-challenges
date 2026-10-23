use std::collections::HashMap;

impl Solution {
    fn degrees(adj: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>, usize) {
        let mut indeg = vec![0; adj.len()];
        let outdeg: Vec<usize> = adj.iter().map(Vec::len).collect();
        for &v in adj.iter().flatten() {
            indeg[v] += 1;
        }
        let m = outdeg.iter().sum();
        (indeg, outdeg, m)
    }

    fn directed_hierholzer(mut adj: Vec<Vec<usize>>, start: usize, edges: usize) -> Vec<usize> {
        let mut stack = vec![start];
        let mut path = Vec::with_capacity(edges + 1);

        while let Some(&v) = stack.last() {
            if let Some(u) = adj[v].pop() {
                stack.push(u);
            } else {
                stack.pop();
                path.push(v);
            }
        }
        path.reverse();
        path
    }

    fn eulerian_path(adj: Vec<Vec<usize>>) -> Vec<usize> {
        let (indeg, outdeg, m) = Self::degrees(&adj);
        let start = (0..adj.len())
            .find(|&v| outdeg[v] == indeg[v] + 1)
            .or_else(|| (0..adj.len()).find(|&v| outdeg[v] > 0))
            .unwrap();
        Self::directed_hierholzer(adj, start, m)
    }

    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut id: HashMap<i32, usize> = HashMap::with_capacity(pairs.len() * 2);
        let mut vals: Vec<i32> = Vec::new();
        let mut adj: Vec<Vec<usize>> = Vec::new();

        for p in &pairs {
            let (a, b) = (p[0], p[1]);
            let u = *id.entry(a).or_insert_with(|| {
                vals.push(a);
                adj.push(Vec::new());
                vals.len() - 1
            });
            let v = *id.entry(b).or_insert_with(|| {
                vals.push(b);
                adj.push(Vec::new());
                vals.len() - 1
            });
            adj[u].push(v);
        }

        Self::eulerian_path(adj)
            .windows(2)
            .map(|w| vec![vals[w[0]], vals[w[1]]])
            .collect()
    }
}
