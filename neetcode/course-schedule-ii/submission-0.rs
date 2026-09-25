impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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
        let mut res = Vec::with_capacity(n);

        while let Some(node) = queue.pop_front() {
            res.push(node as i32);
            for &next in &graph[node] {
                in_degrees[next] -= 1;
                if in_degrees[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if res.len() == n {
            res
        } else {
            vec![]
        }
    }
}
