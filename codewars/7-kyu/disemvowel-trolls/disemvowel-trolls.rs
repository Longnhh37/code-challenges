const VOWELS: [u8; 5] = [b'a', b'i', b'u', b'o', b'e'];
​
fn disemvowel(s: &str) -> String {
    String::from_utf8(
        s
        .bytes()
        .filter(|&b| !VOWELS.contains(&b.to_ascii_lowercase()))
        .collect::<Vec<_>>()
    ).unwrap()
}