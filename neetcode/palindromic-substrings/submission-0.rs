impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut count = 0;
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            count += Self::expand_palindrome(&bytes, i);
        }
        count

    }
    
    fn expand_palindrome(bytes: &[u8], i: usize) -> i32 {
        let mut count = 0_i32;
        // odd palindrome
        let (mut l, mut r) = (i, i);
        while l >= 0 && r < bytes.len() {
            if bytes[l] == bytes[r] {
                count += 1;
                r += 1;
                l = match l.checked_sub(1) {
                    None => break,
                    _ => l - 1,
                };
            } else {
                break;
            }
        }
        // even palindrome
        let (mut l, mut r) = (i, i + 1);
        while l >= 0 && r < bytes.len() {
            if bytes[l] == bytes[r] {
                count += 1;
                r += 1;
                l = match l.checked_sub(1) {
                    None => break,
                    _ => l - 1,
                };
            } else {
                break;
            }
        }

        count
    }
}
