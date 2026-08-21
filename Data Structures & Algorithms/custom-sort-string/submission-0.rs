use std::cmp::Reverse;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut counter = [0_usize; 26];
        for b in s.bytes() {
            counter[(b - b'a') as usize] += 1;
        }

        let mut res = Vec::with_capacity(s.len());
        for b in order.bytes() {
            let i = (b - b'a') as usize;
            let n = counter[i];
            counter[i] = 0;

            if n > 0 {
                res.extend(std::iter::repeat(b).take(n));
            }
        }
        for (i, &cnt) in counter.iter().enumerate() {
            if cnt > 0 {
                let b = b'a' + i as u8;
                res.extend(std::iter::repeat(b).take(cnt as usize));
            }
        }

        String::from_utf8(res).unwrap()
    }
}
