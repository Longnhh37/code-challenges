impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        } else if s.len() > t.len() {
            return false;
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;

        for j in 0..t.len() {
            if s[i] == t[j] {
                i += 1;
            }
            if i == s.len() {
                return true;
            }
        }

        false
    }
}
