impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for b in s.bytes() {
            match b {
                b'(' | b'[' | b'{' => stack.push(b),
                b')' if matches!(stack.pop(), Some(b'(')) => continue,
                b']' if matches!(stack.pop(), Some(b'[')) => continue,
                b'}' if matches!(stack.pop(), Some(b'{')) => continue,
                _ => return false,
            }
        }

        stack.is_empty()
    }
}


