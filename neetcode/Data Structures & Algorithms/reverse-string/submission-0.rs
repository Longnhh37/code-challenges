impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut r = s.len() - 1;
        for l in 0..s.len() / 2 {
            s.swap(l, r);
            r -= 1;
        }
    }
}
