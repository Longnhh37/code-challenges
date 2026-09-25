impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let (mut longest, mut start_idx) = (0, 0);

        for i in 0..b.len() {
            let (cur_len, cur_idx) = Self::expand_palindrome(&b, i);
            if cur_len > longest {
                longest = cur_len;
                start_idx = cur_idx;
            }
        }

        s[start_idx..start_idx + longest].to_string()
    }


    fn expand_palindrome(b: &[u8], i: usize) -> (usize, usize) {
        let odd = Self::expand_from(b, i, i);
        let even = Self::expand_from(b, i, i + 1);
        if odd.0 > even.0 { odd } else { even }
    }

    fn expand_from(b: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
        let (mut longest, mut start_idx) = (0, 0);
        while r < b.len() && b[l] == b[r] {
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
}
