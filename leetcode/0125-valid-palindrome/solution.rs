impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len == 0 {
            return true;
        }

        let mut l = 0;
        let mut r = len - 1;

        while l < r {
            while l < r && !bytes[l].is_ascii_alphanumeric() {
                l += 1;
            }
            while l < r && !bytes[r].is_ascii_alphanumeric() {
                r -= 1;
            }

            if !bytes[l].eq_ignore_ascii_case(&bytes[r]){
                return false;
            }

            l += 1;
            r = r.saturating_sub(1);
        }

        true
    }
}
