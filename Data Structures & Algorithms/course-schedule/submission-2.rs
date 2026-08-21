use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n);
        let mut in_degree = vec![0i32; n];

        for p in &prerequisites {
            let (course, preq) = (p[0] as usize, p[1] as usize);
            map.entry(preq).or_insert_with(Vec::new).push(course);
            in_degree[course] += 1;
        }

        let mut q = VecDeque::new();
        for (course, &cnt) in in_degree.iter().enumerate() {
            if cnt == 0 {
                q.push_back(course);
            }
        }

        let mut visited = 0;
        while let Some(course) = q.pop_front() {
            visited += 1;
            if let Some(preq_list) = map.get(&course) {
                for &c in preq_list {
                    in_degree[c] -= 1;
                    if in_degree[c] == 0 {
                        q.push_back(c);
                    }
                }
            }
        }

        visited == num_courses
    }
}
