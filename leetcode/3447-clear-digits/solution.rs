impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res = Vec::new();

        for b in s.bytes() {
            match b {
                b if b.is_ascii_digit() => { res.pop(); }
                b => res.push(b),
            }
        }

        String::from_utf8(res).unwrap()
    }
}
