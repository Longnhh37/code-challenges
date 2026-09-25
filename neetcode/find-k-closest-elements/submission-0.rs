use std::collections::BinaryHeap;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut max_heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(k);
        
        for &n in &arr {
            let d = (n - x).abs();
            max_heap.push((d, n));
            if max_heap.len() > k {
                max_heap.pop();
            }
        }

        let mut ans: Vec<i32> = max_heap
            .into_vec()
            .into_iter()
            .map(|(_, n)| n)
            .collect();
        
        ans.sort_unstable();
        ans
    }
}

