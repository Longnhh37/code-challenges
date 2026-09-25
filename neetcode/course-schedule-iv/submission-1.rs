use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        let mut adj_rev = vec![HashSet::new(); n];
        let mut in_degree = vec![0; n];

        for course in &prerequisites {
            let (a, b) = (course[0] as usize, course[1] as usize);
            adj[a].push(b);
            adj_rev[b].insert(a);
            in_degree[b] += 1;
        }

        let mut queue: VecDeque::<usize> = (0..n)
            .filter(|&c| in_degree[c] == 0)
            .collect();

        while let Some(course) = queue.pop_front() {
            for &next in &adj[course] {
                let preq = adj_rev[course].clone();
                adj_rev[next].extend(preq);
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        let mut res = vec![false; queries.len()];
        for (i, query) in queries.iter().enumerate() {
            let (pre, course) = (query[0] as usize, query[1] as usize);
            if adj_rev[course].contains(&pre) {
                res[i] = true;
            }
        }

        res
    }
}
