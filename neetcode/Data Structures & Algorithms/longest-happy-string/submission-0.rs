use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut res = Vec::new();
        let mut heap = BinaryHeap::new();

        if a > 0 { heap.push((a, b'a')); }
        if b > 0 { heap.push((b, b'b')); }
        if c > 0 { heap.push((c, b'c')); }

        while let Some((cnt, b)) = heap.pop() {
            let n = res.len();
            if n > 1 && res[n - 2] == b && res[n - 1] == b {
                if let Some((cnt2, b2)) = heap.pop() {
                    res.push(b2);
                    if cnt2 - 1 > 0 {
                        heap.push((cnt2 - 1, b2));
                    }
                    heap.push((cnt, b));
                } else {
                    break;
                }
            } else {
                res.push(b);
                if cnt - 1 > 0 {
                    heap.push((cnt - 1, b));
                }
            }
        }

        String::from_utf8(res).unwrap()

    }
}
