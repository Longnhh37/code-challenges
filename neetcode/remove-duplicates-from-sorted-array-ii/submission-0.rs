use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();
        for (i, &n) in nums.iter().enumerate() {
            if map.contains_key(&n) {
                *map.get_mut(&n).unwrap() += 1;
            } else {
                map.insert(n, 1);
                heap.push(Reverse((i, n)));
            }
        }       

        let mut i = 0;
        let mut res = 0;
        while let Some(Reverse((_, n))) = heap.pop() {
            let cnt = map.get(&n).unwrap();
            if *cnt == 1 {
                nums[i] = n;
                res += 1;
            } else {
                nums[i] = n;
                nums[i + 1] = n;
                i += 1;
                res += 2;
            }
            i += 1;
        }
        res
    }
}