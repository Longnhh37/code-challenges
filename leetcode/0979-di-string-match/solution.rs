impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let (mut l, mut r) = (0, s.len() as i32);
        let mut res = Vec::with_capacity(s.len() + 1);

        for b in s.bytes() {
            match b {
                b'I' => {
                    res.push(l);
                    l += 1;
                }
                _ => {
                    res.push(r);
                    r -= 1;
                }
            }
        } 

        if l < r {
            res.push(l);
        } else {
            res.push(r);
        }
        res
    }
}
