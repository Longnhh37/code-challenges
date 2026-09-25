use std::collections::VecDeque;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        let mut in_degree = vec![0u32; n];
        let mut reach = vec![0u128; n];

        for p in &prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            adj[a].push(b);
            in_degree[b] += 1;
        }

        let mut queue: VecDeque<usize> = (0..n)
            .filter(|&c| in_degree[c] == 0)
            .collect();

        while let Some(course) = queue.pop_front() {
            let mask = reach[course] | (1u128 << course);
            for &next in &adj[course] {
                reach[next] |= mask;
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        queries.iter()
            .map(|q| {
                let (pre, course) = (q[0] as usize, q[1] as usize);
                (reach[course] >> pre) & 1 == 1
            })
            .collect()
    }
}
