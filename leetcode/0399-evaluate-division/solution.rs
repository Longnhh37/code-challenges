use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut id: HashMap<&str, usize> = HashMap::new();
        for s in equations.iter().flatten() {
            let next = id.len();
            id.entry(s.as_str()).or_insert(next);
        }

        let n = id.len();
        let mut adj = vec![Vec::new(); n];
        for (eq, &v) in equations.iter().zip(&values) {
            let (a, b) = (id[eq[0].as_str()], id[eq[1].as_str()]);
            adj[a].push((b, v));
            adj[b].push((a, 1.0 / v));
        }

        let mut comp = vec![usize::MAX; n];
        let mut ratio = vec![0.0f64; n];

        for root in 0..n {
            if comp[root] != usize::MAX {
                continue;
            }
            comp[root] = root;
            ratio[root] = 1.0;
            let mut q = VecDeque::from([root]);

            while let Some(u) = q.pop_front() {
                for &(v, w) in &adj[u] {
                    if comp[v] == usize::MAX {
                        comp[v] = root;
                        ratio[v] = ratio[u] * w;
                        q.push_back(v);
                    }
                }
            }
        }

        queries
            .iter()
            .map(|q| {
                match (id.get(q[0].as_str()), id.get(q[1].as_str())) {
                    (Some(&s), Some(&t)) if comp[s] == comp[t] => ratio[t] / ratio[s],
                    _ => -1.0,
                }
            })
            .collect()
    }
}


