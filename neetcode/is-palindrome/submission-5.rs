impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        let (mut l, mut r) = (0, b.len() - 1);

        while l < r {
            while l < r && !b[l].is_ascii_alphanumeric() {
                l += 1;
            }
            while r > l && !b[r].is_ascii_alphanumeric() {
                r -= 1;
            }
            if l >= r {
                break;
            }

            if b[l].to_ascii_lowercase() != b[r].to_ascii_lowercase() {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
