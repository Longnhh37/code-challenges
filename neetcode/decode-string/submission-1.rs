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
                let mut repeat: usize = 0;
                let mut place: usize = 1;

                while let Some(&d) = stack.last() {
                    if !d.is_ascii_digit() {
                        break;
                    }
                    stack.pop();
                    repeat += (d - b'0') as usize * place;
                    place *= 10;
                }

                stack.reserve(msg.len() * repeat);
                for _ in 0..repeat {
                    stack.extend_from_slice(&msg);
                }
            } else {
                stack.push(b);
            }
        }

        String::from_utf8(stack).unwrap()
    }
}
