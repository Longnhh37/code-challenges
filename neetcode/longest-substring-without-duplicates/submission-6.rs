use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let b = s.as_bytes();
        let mut res = 1;
        let mut l = 0;
        let mut seen = HashSet::new();

        for r in 0..b.len() {
            if !seen.insert(b[r]) {
                while b[l] != b[r] {
                    seen.remove(&b[l]);
                    l += 1;
                }
                l += 1;
            }
            res = res.max(seen.len() as i32);
        }

        res
    }
}
