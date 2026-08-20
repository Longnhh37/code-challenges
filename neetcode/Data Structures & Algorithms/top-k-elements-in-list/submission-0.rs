use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

#[derive(Eq, PartialEq)]
struct Pair {
    num: i32,
    freq: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freqs: HashMap<i32, i32> = HashMap::new();
        for &n in &nums {
            *freqs.entry(n).or_insert(0) += 1;
        }

        let mut min_heap: BinaryHeap<Pair> = BinaryHeap::new();


        let k = k as usize;
        for (num, freq) in freqs {
            min_heap.push(Pair { num, freq });
            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        let mut ans = Vec::with_capacity(k);
        while let Some(pair) = min_heap.pop() {
            ans.push(pair.num);
        }

        ans
    }
}
