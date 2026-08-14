use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counter = vec![0; 128];
        for b in s.bytes() {
            counter[b as usize] += 1;
        }

        let mut heap = counter
            .into_iter()
            .enumerate()
            .skip(47) // 48 == ASCII '0'
            .filter(|&(_, cnt)| cnt > 0)
            .fold(BinaryHeap::new(), |mut acc, (b, cnt)| {
                acc.push((cnt, b));
                acc
            });

        let mut res = Vec::with_capacity(s.len());
        while let Some((cnt, b)) = heap.pop() {
            res.extend(vec![b as u8; cnt as usize]);
        }
        String::from_utf8(res).unwrap()
    }
}
