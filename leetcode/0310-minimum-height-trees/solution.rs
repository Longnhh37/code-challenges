use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n <= 2 {
            return (0..n).collect();
        }

        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        let mut in_deg = vec![0usize; n];

        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
            in_deg[a] += 1;
            in_deg[b] += 1;
        }

        let mut remaining = n;
        let mut q: VecDeque<_> = in_deg
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| (c == 1).then_some(i))
            .collect();

        while remaining > 2 {
            remaining -= q.len();
            for _ in 0..q.len() {
                let u = q.pop_front().unwrap();
                for &v in &adj[u] {
                    in_deg[v] -= 1;
                    if in_deg[v] == 1 {
                        q.push_back(v);
                    }
                }
            }
        }

        q.into_iter().map(|x| x as i32).collect()
    }
}

