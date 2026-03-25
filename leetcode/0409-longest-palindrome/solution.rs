use std::collections::HashSet;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut seen: HashSet<char> = HashSet::new();
        let mut length = 0;

        for c in s.chars() {
            if !seen.insert(c) {
                seen.remove(&c);
                length += 2;
            }
        }
        length + (seen.len() > 0) as i32
    }
}
