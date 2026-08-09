impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::helper(&s) == Self::helper(&t)
    }

    fn helper(s: &str) -> Vec<u8> {
        let mut res = Vec::new();
        for b in s.bytes() {
            match b {
                b'#' => { res.pop(); }
                b => res.push(b),
            }
        }
        res
    }
}
