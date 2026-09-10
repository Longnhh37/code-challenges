use std::collections::BinaryHeap;

impl Solution {
    pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        let mut heaps: Vec<BinaryHeap<i32>> = nums.iter()
            .map(|vec| {
                let mut heap = BinaryHeap::new();
                for &v in vec {
                    heap.push(v);
                }
                heap
            })
            .collect();

        let mut empty = false;
        let mut score = 0;

        while !empty {
            empty = true;
            let mut cur_max = i32::MIN;

            for heap in heaps.iter_mut() {
                if let Some(v) = heap.pop() {
                    empty = false;
                    cur_max = cur_max.max(v);
                }
            }

            if cur_max != i32::MIN {
                score += cur_max;
            }
        }

        score
    }
}
