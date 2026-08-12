impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for w in &words {
            if Self::is_palindrom(w) {
                return w.to_string();
            }
        }
        String::new()
    }

    fn is_palindrom(s: &str) -> bool {
        let b = s.as_bytes();
        let (mut l, mut r) = (0, b.len() - 1);
        while l < r {
            if b[l] != b[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
