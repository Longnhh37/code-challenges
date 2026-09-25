impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n);
        let mut in_degree = vec![0i32; n];
        let mut res = Vec::with_capacity(n);

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

        let mut finished = 0;
        while let Some(course) = q.pop_front() {
            finished += 1;
            res.push(course as i32);
            if let Some(preq_list) = map.get(&course) {
                for &c in preq_list {
                    in_degree[c] -= 1;
                    if in_degree[c] == 0 {
                        q.push_back(c);
                    }
                }
            }
        }

        if finished == num_courses {
            res
        } else {
            Vec::new()
        }
    }
}
