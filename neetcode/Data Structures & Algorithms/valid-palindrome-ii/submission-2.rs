impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();

        let mut l = 0;
        let mut r = bytes.len() - 1;

        while l < r {
            if bytes[l] != bytes[r] {
                return Self::is_pal(&bytes, l + 1, r) || Self::is_pal(&bytes, l, r - 1);
                }
            l += 1;
            r -= 1;
        }
        true
    }

    fn is_pal(bytes: &[u8], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if bytes[l] != bytes[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
