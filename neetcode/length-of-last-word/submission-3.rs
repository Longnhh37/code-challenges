impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut s = s.as_bytes();
        let mut r = s.len() - 1;
        while r > 0 && s[r] == b' ' {
            r -= 1;
        }
        if r == 0 {
            return r as i32 + 1;
        }
        let mut l = r - 1;
        while l > 0 && s[l] != b' ' {
            l -= 1;
        }

        (r - l) as i32
    }
}
