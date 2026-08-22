use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {

        let mut adj: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for (i, eq) in equations.iter().enumerate() {
            let a = &eq[0];
            let b = &eq[1];
            let v = values[i];

            adj.entry(a.clone()).or_default().push((b.clone(), v));
            adj.entry(b.clone()).or_default().push((a.clone(), 1.0 / v));
        }

        queries
            .iter()
            .map(|q| {
                let mut visited = HashSet::new();
                Self::dfs(&q[0], &q[1], &adj, &mut visited)
            })
            .collect()
    }

    fn dfs(
        src: &str,
        target: &str, 
        adj: &HashMap<String, Vec<(String, f64)>>,
        visited: &mut HashSet<String>,
    ) -> f64 {
        if !adj.contains_key(src) || !adj.contains_key(target) {
            return -1.0;
        }
        if src == target {
            return 1.0;
        }
        visited.insert(src.to_string());
        if let Some(neighbors) = adj.get(src) {
            for (nei, weight) in neighbors {
                if !visited.contains(nei) {
                    let result = Self::dfs(nei, target, adj, visited);
                    if result != -1.0 {
                        return weight * result;
                    }
                }
            }
        }

        -1.0
    }
}