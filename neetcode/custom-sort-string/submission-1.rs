impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut counter = [0_usize; 26];
        for b in s.bytes() {
            counter[(b - b'a') as usize] += 1;
        }

        let mut res = Vec::with_capacity(s.len());
        for b in order.bytes() {
            let i = (b - b'a') as usize;
            let cnt = counter[i];
            counter[i] = 0;

            if cnt > 0 {
                res.extend(vec![b; cnt]);
            }
        }

        for (i, &cnt) in counter.iter().enumerate() {
            if cnt > 0 {
                let b = b'a' + i as u8;
                res.extend(vec![b; cnt]);
            }
        }

        String::from_utf8(res).unwrap()
    }
}
