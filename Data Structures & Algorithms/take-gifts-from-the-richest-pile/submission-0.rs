use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap: BinaryHeap<i32> = gifts
            .iter()
            .fold(BinaryHeap::new(), |mut acc, &x| {
                acc.push(x);
                acc
            });
        for _ in 0..k {
            let mut v = heap.pop().unwrap();
            v = v.isqrt();
            heap.push(v);
        }
        heap.into_iter().map(|n| n as i64).sum()
    }
}
