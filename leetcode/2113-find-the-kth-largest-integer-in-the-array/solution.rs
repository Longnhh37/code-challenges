use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Wrap(String);

impl PartialOrd for Wrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Wrap {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.len() > other.0.len() {
            return Ordering::Greater;
        } else if self.0.len() < other.0.len() {
            return Ordering::Less;
        } else {
            return self.0.cmp(&other.0);
        }
    }
}

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let k = k as usize;
        let mut min_heap = BinaryHeap::with_capacity(k + 1);
        for n in &nums {
            min_heap.push(Reverse(Wrap(n.to_string())));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        min_heap.pop().unwrap().0.0
    }
}
