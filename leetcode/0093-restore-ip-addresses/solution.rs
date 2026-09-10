impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let chars: Vec<u8> = s.into_bytes();
        let mut res = Vec::new();
        Self::backtrack(&chars, 0, &mut Vec::new(), &mut res);
        res
    }
    
    fn backtrack(chars: &[u8], start: usize, path: &mut Vec<String>, res: &mut Vec<String>) {
        if path.len() == 4 {
            if start == chars.len() {
                res.push(path.join("."));
            }
            return;
        }

        let remaining_len = chars.len() - start;
        let remaining_segments = 4 - path.len();
        if remaining_len < remaining_segments || remaining_len > remaining_segments * 3 {
            return;
        }

        for len in 1..=3.min(chars.len() - start) {
            let end = start + len;
            let segment = &chars[start..end];

            if !Self::is_valid_segment(segment) {
                continue;
            }

            path.push(String::from_utf8(segment.to_vec()).unwrap());
            Self::backtrack(chars, end, path, res);
            path.pop();
        }
    }

    fn is_valid_segment(segment: &[u8]) -> bool {
        if segment.len() > 1 && segment[0] == b'0' {
            return false;
        }
        let value: u32 = segment.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as u32);
        value <= 255
    }
}
