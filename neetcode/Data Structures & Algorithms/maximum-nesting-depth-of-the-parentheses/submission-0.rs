impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut open = 0;
        let mut res = 0;
        for b in s.bytes() {
            match b {
                b'(' => {
                    open += 1;
                    res = res.max(open);
                }
                b')' => open -= 1,
                _ => {},
            }
        }
        
        res
    }
}
