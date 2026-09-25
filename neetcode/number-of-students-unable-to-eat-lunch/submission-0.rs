use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students: VecDeque<_> = students.into_iter().collect();

        let mut i = 0;
        let mut j = 0;
        while i < students.len() {
            if students[0] == sandwiches[j] {
                students.pop_front();
                j += 1;
                i = 0;
            } else {
                let stu = students.pop_front().unwrap();
                students.push_back(stu);
                i += 1;
            }
        }

        students.len() as i32
    }
}