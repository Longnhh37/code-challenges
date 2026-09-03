use std::collections::BinaryHeap;
use std::cmp::Reverse;
​
fn sjf(jobs: &[usize], index: usize) -> usize {
    let mut heap = BinaryHeap::new();
    for (i, &n) in jobs.iter().enumerate() {
        heap.push(Reverse((n, i)));
    }
    
    let mut cc = 0;
    while let Some(Reverse((n, i))) = heap.pop() {
        cc += n;
        if i == index {
            break;
        }
    }
    
    cc
}