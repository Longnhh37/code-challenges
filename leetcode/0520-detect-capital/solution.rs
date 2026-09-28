impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let bytes = word.as_bytes();
        bytes.iter().all(|b| b.is_ascii_uppercase())
        || bytes.iter().all(|b| b.is_ascii_lowercase())
        || (bytes[0].is_ascii_uppercase() && bytes[1..].iter().all(|b| b.is_ascii_lowercase()))
    }
}
