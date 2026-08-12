impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new();
        for b in s.bytes() {
            match b {
                b'B' if stack.last() == Some(&b'A') => { stack.pop(); }
                b'D' if stack.last() == Some(&b'C') => { stack.pop(); }
                b => stack.push(b),
            }
        }
        stack.len() as i32
    }
}
