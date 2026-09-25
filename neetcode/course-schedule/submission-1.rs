use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut in_degrees = vec![0_usize; n];

        for edge in &prerequisites {
            let [course, preq] = edge[..] else { continue };
            let (course, preq) = (course as usize, preq as usize);
            graph[preq].push(course);
            in_degrees[course] += 1;
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degrees[i] == 0).collect();
        let mut finished = 0;

        while let Some(node) = queue.pop_front() {
            finished += 1;
            for &next in &graph[node] {
                    in_degrees[next] -= 1;
                    if in_degrees[next] == 0 {
                        queue.push_back(next);
                    }
                }
            }

        finished == n
    }
}
