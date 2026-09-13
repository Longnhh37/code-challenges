use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        let mut in_deg = vec![0; n];

        for pre in &prerequisites {
            let (v, u) = (pre[0] as usize, pre[1] as usize);
            adj[u].push(v);
            in_deg[v] += 1;
        }

        let mut q: VecDeque<usize> = in_deg
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 0 { Some(i) } else { None })
            .collect();

        let mut res = Vec::new();
        while let Some(cur) = q.pop_front() {
            res.push(cur as i32);
            for &nei in &adj[cur] {
                in_deg[nei] -= 1;
                if in_deg[nei] == 0 {
                    q.push_back(nei);
                }
            }
        }

        if res.len() == n {
            res 
        } else {
            Vec::new() 
        }
    }
}
