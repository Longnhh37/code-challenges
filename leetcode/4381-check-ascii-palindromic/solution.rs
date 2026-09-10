impl Solution {
    pub fn is_palindromic(s: String) -> bool {
        let bytes: Vec<u8> = s.bytes()
            .flat_map(|b| format!("{:08b}", b).into_bytes())
            .collect();

        for i in 0..bytes.len() / 2 {
            if bytes[i] != bytes[bytes.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}
