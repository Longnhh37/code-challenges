impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut l = 0;
        let mut r = len - 1;

        while l < r {
            while l < len && !chars[l].is_ascii_alphanumeric() {
                l += 1;
            }
            while r > 0 && !chars[r].is_ascii_alphanumeric() {
                r -= 1;
            }

            if l >= r {
                break;
            }

            if chars[l].to_ascii_lowercase() != chars[r].to_ascii_lowercase() {
                return false;
            }

            l += 1;
            r -= 1;
        }

        true
    }
}
