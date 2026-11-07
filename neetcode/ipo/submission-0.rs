use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        projects.sort_unstable();

        let mut w = w as i64;
        let mut heap = BinaryHeap::new();
        let mut i = 0;

        for _ in 0..k {
            while i < projects.len() && projects[i].0 as i64 <= w {
                heap.push(projects[i].1);
                i += 1;
            }
            match heap.pop() {
                Some(p) => w += p as i64,
                None => break,
            }
        }

        w as i32
    }
}
