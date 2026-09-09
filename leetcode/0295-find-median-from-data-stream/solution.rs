use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Default)]
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}


impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }
    
    fn add_num(&mut self, num: i32) {
        self.min_heap.push(Reverse(num));
        let Reverse(v) = self.min_heap.pop().unwrap();
        self.max_heap.push(v);   

        // rebalancing: min_heap >= max_heap
        if self.max_heap.len() > self.min_heap.len() {
            let v = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(v));
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.min_heap.len() == self.max_heap.len() {
            let v1 = self.max_heap.peek().unwrap_or(&0);
            let Reverse(v2) = self.min_heap.peek().unwrap_or(&Reverse(0));
            (v1 + v2) as f64 / 2.0
        } else {
            let Reverse(v) = *self.min_heap.peek().unwrap_or(&Reverse(0));
            v as f64
        }
    }
}

