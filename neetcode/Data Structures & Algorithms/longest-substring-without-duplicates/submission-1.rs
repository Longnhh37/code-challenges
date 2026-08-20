use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut res = 0;
        let mut start = 0;
        let mut seen = HashSet::new();

        for (end, &b) in bytes.iter().enumerate() {
            if !seen.insert(b) {
                res = res.max(end - start);
                while bytes[start] != b {
                    seen.remove(&bytes[start]);
                    start += 1;
                }
                start += 1;
            }
        }

        res.max(seen.len()) as i32
    }
}
