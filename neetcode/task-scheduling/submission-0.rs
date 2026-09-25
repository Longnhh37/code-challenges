use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0_i32; 26];
        for &task in &tasks {
            freq[(task as u8 - b'A') as usize] += 1;
        }
        freq.sort_unstable();

        let maxf = freq[25];
        let mut idle = (maxf - 1) * n;

        for i in (0..25).rev() {
            idle -= (maxf - 1).min(freq[i]);
        }
        0.max(idle) + tasks.len() as i32

    }
}
