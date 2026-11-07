use std::collections::VecDeque;

impl Solution {
    fn topo_sort(k: usize, conditions: &[Vec<i32>]) -> Option<Vec<usize>> {
        let mut adj = vec![Vec::new(); k + 1];
        let mut indeg = vec![0usize; k + 1];
        for c in conditions {
            let (a, b) = (c[0] as usize, c[1] as usize);
            adj[a].push(b);
            indeg[b] += 1;
        }

        let mut q: VecDeque<usize> = (1..=k).filter(|&v| indeg[v] == 0).collect();
        let mut order = Vec::with_capacity(k);
        while let Some(u) = q.pop_front() {
            order.push(u);
            for &v in &adj[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        (order.len() == k).then_some(order)
    }

    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>)
    -> Vec<Vec<i32>> {
        let k = k as usize;
        let (Some(row_order), Some(col_order)) = (
            Self::topo_sort(k, &row_conditions),
            Self::topo_sort(k, &col_conditions),
        ) else {
            return vec![];
        };

        let mut row_pos = vec![0usize; k + 1];
        let mut col_pos = vec![0usize; k + 1];
        for (i, &v) in row_order.iter().enumerate() {
            row_pos[v] = i;
        }
        for (i, &v) in col_order.iter().enumerate() {
            col_pos[v] = i;
        }

        let mut matrix = vec![vec![0; k]; k];
        for v in 1..=k {
            matrix[row_pos[v]][col_pos[v]] = v as i32;
        }

        matrix
    }
}
