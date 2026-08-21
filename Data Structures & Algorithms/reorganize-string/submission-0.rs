use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut res: Vec<u8> = Vec::with_capacity(s.len());
        let mut prev: Option<(i32, u8)> = None;

        let mut counter = [0_i32; 26];
        for b in s.bytes() {
            counter[(b - b'a') as usize] += 1;
        }

        let mut heap = counter
            .into_iter()
            .enumerate()
            .filter(|(b, cnt)| *cnt > 0)
            .fold(BinaryHeap::new(), |mut acc, (b, cnt)| {
                acc.push((cnt, b as u8));
                acc
            });

        loop {
            if let Some((mut cnt, b)) = heap.pop() {
                res.push(b'a' + b);
                cnt -= 1;

                if let Some(p) = prev.take() {
                    heap.push(p);
                }

                if cnt > 0 {
                    prev = Some((cnt, b));
                }
                
            } else if prev.is_some() {
                return String::new();
            } else {
                break;
            }
        }

        String::from_utf8(res).unwrap()
    }
}
