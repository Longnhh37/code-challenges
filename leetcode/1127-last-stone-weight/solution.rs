use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<_> = stones.into_iter().collect();       
        while let Some(s1) = heap.pop() {
            match heap.pop() {
                None => return s1,
                Some(s2) => {
                    if s1 - s2 != 0 {
                        heap.push(s1 - s2);
                    }
                }
            }
        }
        0
    }
}
