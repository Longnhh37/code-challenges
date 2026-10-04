use std::collections::VecDeque;

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut adj = vec![Vec::new(); n];
        let mut in_deg = vec![0u32; n]; 

        for r in &richer {
            let (u, v) = (r[0] as usize, r[1] as usize);
            adj[u].push(v);
            in_deg[v] += 1;
        }

        let mut ans: Vec<usize> = (0..n).collect();
        let mut q: VecDeque<usize> = (0..n).filter(|&i| in_deg[i] == 0).collect();

        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if quiet[ans[u]] < quiet[ans[v]] {
                    ans[v] = ans[u];
                }
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        ans.into_iter().map(|x| x as i32).collect()
    }
}
