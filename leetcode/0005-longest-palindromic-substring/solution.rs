impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let (mut start, mut longest) = (0, 0);

        for i in 0..bytes.len() {
            let (cur, cur_len) = Self::expand(&bytes, i);
            if cur_len > longest {
                (start, longest) = (cur, cur_len);
            }
        } 

        s[start..start + longest].to_string()
    }

    fn expand(bytes: &[u8], start: usize) -> (usize, usize) {
        let odd = Self::expand_from(bytes, start, start);
        let even = Self::expand_from(bytes, start, start + 1);
        if odd.1 > even.1 {
            odd
        } else {
            even
        }
    }

    fn expand_from(bytes: &[u8], start: usize, start2: usize) -> (usize, usize) {
        let mut l = start as isize;
        let mut r = start2;

        while l >= 0 && r < bytes.len() {
            if bytes[l as usize] == bytes[r] {
                l -= 1;
                r += 1;
            } else {
                break;
            }
        }
        let l = (l + 1) as usize;
        (l, r - l)
    }
}
