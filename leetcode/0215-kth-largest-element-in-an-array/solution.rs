use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);

        for &n in &nums {
            if heap.len() < k {
                heap.push(Reverse(n));
            } else {
                heap.push(Reverse(n));
                heap.pop();
            }
        }
        let Reverse(n) = heap.pop().unwrap();
        n
    }
}
