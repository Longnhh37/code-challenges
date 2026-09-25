impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<u8> = Vec::new();
        
        for b in s.bytes() {
            if b == b']' {
                // get msg
                let mut msg: Vec<u8> = Vec::new();
                while let Some(b) = stack.pop() && b != b'[' {
                    msg.push(b);
                }
                msg.reverse();

                // get repeated number
                let mut repeat: Vec<u8> = Vec::new();
                while let Some(b) = stack.last() && b.is_ascii_digit() {
                    repeat.push(stack.pop().unwrap());
                }
                repeat.reverse();
                let repeat: usize = String::from_utf8(repeat).unwrap().parse().unwrap();
                let msg: Vec<_> = msg
                    .iter()
                    .cycle()
                    .take(msg.len() * repeat)
                    .cloned()
                    .collect();
                stack.extend_from_slice(&msg);
            } else {
                stack.push(b);
            }
        }

        String::from_utf8(stack).unwrap()
    }
}
