impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = Vec::new();
        for b in s.bytes() {
            if b.is_ascii_digit() {
                stack.pop();
            } else {
                stack.push(b);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}
