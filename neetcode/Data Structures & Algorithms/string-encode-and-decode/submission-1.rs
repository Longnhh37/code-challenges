impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let total_len: usize = strs.iter().map(|s| s.len() + 12).sum();
        let mut encoded = String::with_capacity(total_len);

        for s in &strs {
            encoded.push_str(&s.len().to_string());
            encoded.push('#');
            encoded.push_str(s);
        }
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut decoded = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            let mut len: usize = 0;

            while bytes[i] != b'#' {
                len = len * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            i += 1; 
            let slice = &bytes[i..i + len];
            decoded.push(String::from_utf8(slice.to_vec()).unwrap());

            i += len;
        }

        decoded
    }
}
