pub fn encode(key: &str, s: &str) -> Option<String> {
    let key_bytes = key.as_bytes();
    if key_bytes.is_empty()
        || !key_bytes.iter().all(|b| b.is_ascii_lowercase())
        || !s.bytes().all(|b| b.is_ascii_lowercase())
    {
        return None;
    }

    let key_len = key_bytes.len();

    let out = s
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let p = b - b'a';
            let k = key_bytes[i % key_len] - b'a';
            (b'a' + (p + k) % 26) as char
        })
        .collect();

    Some(out)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let key_bytes = key.as_bytes();
    if key_bytes.is_empty()
        || !key_bytes.iter().all(|b| b.is_ascii_lowercase())
        || !s.bytes().all(|b| b.is_ascii_lowercase())
    {
        return None;
    }

    let key_len = key_bytes.len();

    let out = s
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let c = b - b'a';
            let k = key_bytes[i % key_len] - b'a';
            (b'a' + (c + 26 - k) % 26) as char
        })
        .collect();

    Some(out)
}
pub fn encode_random(s: &str) -> (String, String) {
    let len = rand::random_range(100..1000);

    let key: String = (0..len)
        .map(|_| (b'a' + rand::random_range(0..=25)) as char)
        .collect();

    (key.clone(), encode(key.as_str(), s).unwrap())
}

fn main() {
    let text = "abcdefghij";
    let key = "abcdefghij";

    dbg!(decode(key, text));
}
