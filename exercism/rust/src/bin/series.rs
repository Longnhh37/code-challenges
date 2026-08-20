pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() || len == 0 {
        return vec![];
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|w| String::from_utf8(w.to_vec()).unwrap())
        .collect()
}
fn main() {}
