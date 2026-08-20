pub fn rotate(input: &str, key: u8) -> String {
    input
        .bytes()
        .map(|b| match b {
            b'a'..=b'z' => ((b - b'a' + key) % 26 + b'a') as char,
            b'A'..=b'Z' => ((b - b'A' + key) % 26 + b'A') as char,
            _ => b as char,
        })
        .collect::<String>()
}

fn main() {}
