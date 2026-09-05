use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj = vec![vec![]; n];
        let mut in_deg = vec![0u32; n];

        for preq in &prerequisites {
            let (v, u) = (preq[0] as usize, preq[1] as usize);
            adj[u].push(v);
            in_deg[v] += 1;
        }

        let mut q: VecDeque<usize> = (0..n).filter(|&v| in_deg[v] == 0).collect();
        let mut visited = 0;

        while let Some(cur) = q.pop_front() {
            visited += 1;
            for &nei in &adj[cur] {
                in_deg[nei] -= 1;
                if in_deg[nei] == 0 {
                    q.push_back(nei);
                }
            }
        }

        visited == n
    }
}
