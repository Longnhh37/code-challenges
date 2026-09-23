impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut t = s.into_bytes();

        if t[0] == b'?' && t[1] == b'?' {
            t[0] = b'1';
            t[1] = b'1';
        } else if t[0] == b'?' {
            if t[1] == b'0'|| t[1] == b'1' {
                t[0] = b'1';
            } else {
                t[0] = b'0';
            }
        } else if t[1] == b'?' {
            if t[0] == b'1' {
                t[1] = b'1';
            } else {
                t[1] = b'9';
            }
        }

        if t[3] == b'?' {
            t[3] = b'5';
        }
        if t[4] == b'?' {
            t[4] = b'9';
        }

        String::from_utf8(t).unwrap()
    }
}
