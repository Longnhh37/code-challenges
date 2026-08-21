use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        for (cnt, ch) in [(a, b'a'), (b, b'b'), (c, b'c')] {
            if cnt > 0 {
                heap.push((cnt, ch));
            }
        }

        let mut res = Vec::new();

        while let Some((cnt, ch)) = heap.pop() {
            let would_triple = res.len() >= 2 
                && res[res.len() - 1] == ch
                && res[res.len() - 2] == ch;
            
            if !would_triple {
                res.push(ch);
                if cnt > 1 {
                    heap.push((cnt - 1, ch));
                }
                continue;
            }

            let Some((cnt2, ch2)) = heap.pop() else { break };
            res.push(ch2);
            if cnt2 > 1 {
                heap.push((cnt2 - 1, ch2));
            }
            heap.push((cnt, ch));

        }

        String::from_utf8(res).unwrap()

    }
}
