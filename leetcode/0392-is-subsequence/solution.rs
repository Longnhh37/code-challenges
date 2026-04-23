impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let len = s.len();
        let mut i = 0;

        for v in t.bytes() {
            if i >= len {
                return true;
            }
            if v == s[i] {
                i += 1;
            }
        }

        i >= len
    }
}
