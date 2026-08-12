impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<u8> = Vec::new();

        for cur in s.bytes() {
            if let Some(last) = stack.last() && 
                *last != cur &&
                (cur.to_ascii_lowercase() == *last || cur.to_ascii_uppercase() == *last) {
                    stack.pop();
                } else {
                    stack.push(cur);
                }
        }

        String::from_utf8(stack).unwrap()
    }
}
