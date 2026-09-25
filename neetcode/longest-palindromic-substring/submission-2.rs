impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let (mut longest, mut start_idx) = (0_usize, 0_usize);
        for i in 0..bytes.len() {
            let (cur_len, cur_idx) = Self::expand_palindrome(&bytes, i);
            if cur_len > longest {
                (longest, start_idx) = (cur_len, cur_idx);
            }
        }

        String::from_utf8(bytes[start_idx..start_idx + longest].to_vec()).unwrap()
    }

    fn expand_from(bytes: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
        let mut longest = 0_usize;
        let mut start_idx = 0_usize;
        while r < bytes.len() && bytes[l] == bytes[r] {
            longest = longest.max(r - l + 1);
            start_idx = l;
            r += 1;
            match l.checked_sub(1) {
                Some(v) => l = v,
                None => break,
            }
        }
        (longest, start_idx)
    } 

    fn expand_palindrome(bytes: &[u8], i: usize) -> (usize, usize) {
        let odd = Self::expand_from(bytes, i, i);
        let even = Self::expand_from(bytes, i, i + 1);
        if odd.0 > even.0 { odd } else { even }
    }
}
