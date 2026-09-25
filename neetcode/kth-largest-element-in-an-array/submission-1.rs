use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let k = k as usize;
        for &n in &nums {
            min_heap.push(Reverse(n));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        min_heap.pop().unwrap().0
    }
}
