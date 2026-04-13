use std::collections::HashSet;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut seen = HashSet::new();
        let mut length = 0;

        for b in s.bytes() {
            if !seen.insert(b) {
                seen.remove(&b);
                length += 2;
            }
        }
        length + (seen.len() > 0) as i32
    }
}
