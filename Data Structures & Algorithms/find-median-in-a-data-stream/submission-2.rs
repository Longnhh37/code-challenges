use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        let v = self.max_heap.pop().unwrap();
        self.min_heap.push(Reverse(v));

        let min_len = self.min_heap.len();
        let max_len = self.max_heap.len();

        if min_len > max_len {
            let Reverse(v) = self.min_heap.pop().unwrap();
            self.max_heap.push(v);
        }
    }

    pub fn find_median(&self) -> f64 {
        let max_len = self.max_heap.len();
        let min_len = self.min_heap.len();

        if max_len > min_len {
            *self.max_heap.peek().unwrap() as f64
        } else {
            let a = *self.max_heap.peek().unwrap() as f64;
            let Reverse(b) = *self.min_heap.peek().unwrap();
            (a + b as f64) / 2.0
        }
    }
}
