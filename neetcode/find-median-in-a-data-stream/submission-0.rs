use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    max_heap: BinaryHeap<Reverse<i32>>,
    min_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.max_heap.push(Reverse(num));
        let v = self.max_heap.pop().unwrap().0;
        self.min_heap.push(v);

        let min_len = self.min_heap.len();
        let max_len = self.max_heap.len();

        if max_len > min_len && max_len - min_len == 2 {
            let v = self.max_heap.pop().unwrap().0;
            self.min_heap.push(v);
        } else if min_len > max_len && min_len - max_len == 2 {
            let v = self.min_heap.pop().unwrap();
            self.max_heap.push(Reverse(v));
        }
    }

    pub fn find_median(&self) -> f64 {
        let max_len = self.max_heap.len();
        let min_len = self.min_heap.len();

        let max_v = self.max_heap.peek().unwrap_or(&Reverse(0)).0 as f64;
        let min_v = *self.min_heap.peek().unwrap_or(&0) as f64;

        if max_len > min_len {
            max_v
        } else if min_len > max_len {
            min_v
        } else {
            (max_v + min_v) / 2.0
        }
    }
}
