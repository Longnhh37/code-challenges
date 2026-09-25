impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut res = Vec::new();
        let mut bytes = s.as_bytes();
        let mut l = 0usize;

        while l < bytes.len() {
            let mut r = Self::find_last_pos(&bytes, bytes[l]);
            if l == r {
                res.push(1);
                l += 1;
                continue;
            }

            let mut mid = l + 1;
            while mid < r {
                let b = bytes[mid];
                r = r.max(Self::find_last_pos(&bytes, b));
                mid += 1;
            }
            res.push((r - l + 1) as i32);
            l = r + 1;
        }

        res
    }

    fn find_last_pos(bytes: &[u8], b: u8) -> usize {
        bytes.iter().rposition(|&x| x == b).unwrap()
    }
}
