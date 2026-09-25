use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n);
        let mut in_degrees: Vec<usize> = vec![0; n];

        for elem in &prerequisites {
            let course = elem[0];
            let course = course as usize;
            let preq = elem[1];
            let preq = preq as usize;
            map.entry(preq).or_default().push(course);
            in_degrees[course] += 1;
        }

        let mut queue = VecDeque::new();

        for i in 0..n {
            if in_degrees[i] == 0 {
                queue.push_back(i);
            }
        }
        let mut enrolled_courses = 0;

        while let Some(node) = queue.pop_front() {
            enrolled_courses += 1;
            if let Some(neighbors) = map.get(&node) && !neighbors.is_empty() {
                for &neighbor in neighbors {
                    in_degrees[neighbor] -= 1;
                    if in_degrees[neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        enrolled_courses == n
    }
}
