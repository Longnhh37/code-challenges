impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();
        for s in strs {
            encoded.push_str(&s.chars().count().to_string());
            encoded.push('#');
            encoded.push_str(&s);
        }

        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut it = s.chars();
        let mut decoded = Vec::new();
        let mut len = 0_u64;
        
        while let Some(c) = it.next() {
            if c == '#' {
                let mut s = String::new();
                for _ in 0..len {
                    s.push(it.next().unwrap());
                }
                decoded.push(s);
                len = 0;
                continue;
            }
            len = 10 * len + (c as u8 - b'0') as u64;
        }

        decoded
    }
}
