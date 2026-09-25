use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let n = tasks.len();
        let mut pending: Vec<(i32, i32, i32)> = tasks
            .iter()
            .enumerate()
            .map(|(i, t)| (t[0], t[1], i as i32))
            .collect();
        pending.sort_unstable();

        let mut available = BinaryHeap::new();
        let mut res = Vec::with_capacity(n);
        let mut time: u64 = 0;
        let mut i = 0;

        while !available.is_empty() || i < n {
            while i < n && pending[i].0 as u64 <= time {
                let (_, proc_time, idx) = pending[i];
                available.push(Reverse((proc_time, idx)));
                i += 1;
            }

            if available.is_empty() {
                time = pending[i].0 as u64;
                continue;
            }

            let Reverse((proc_time, idx)) = available.pop().unwrap(); 
            time += proc_time as u64;
            res.push(idx);
            
        }

        res
    }
}
