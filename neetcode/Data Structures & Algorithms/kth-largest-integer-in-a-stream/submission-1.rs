use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut min_heap = BinaryHeap::with_capacity(k + 1);

        for &n in &nums {
            min_heap.push(Reverse(n));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        
        Self { min_heap, k }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));
        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }
        self.min_heap.peek().unwrap().0
    }
}
