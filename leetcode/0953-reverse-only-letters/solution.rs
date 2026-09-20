impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut res = s.into_bytes();
        let (mut l, mut r) = (0, res.len() - 1);

        while l < r {
            while l < r && !res[l].is_ascii_alphabetic() {
                l += 1;
            }
            while l < r && !res[r].is_ascii_alphabetic() {
                r -= 1;
            }
            res.swap(l, r);
            l += 1;
            r -= 1;
        }
        String::from_utf8(res).unwrap()
    }
}
