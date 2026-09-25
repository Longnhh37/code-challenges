use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut max_heap = BinaryHeap::with_capacity(k);

        for p in &points {
            let a = p[0];
            let b = p[1];
            let d = a.pow(2) + b.pow(2);
            max_heap.push((d, a, b));
            if max_heap.len() > k {
                max_heap.pop();
            }
        }

        max_heap
            .into_vec()
            .into_iter()
            .map(|(_, a, b)| vec![a, b]) 
            .collect()
    }
}
