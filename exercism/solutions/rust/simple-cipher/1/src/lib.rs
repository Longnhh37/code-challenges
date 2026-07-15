pub fn encode(key: &str, s: &str) -> Option<String> {
    let key_bytes = key.as_bytes();
    if key_bytes.is_empty() || !key_bytes.iter().all(|b| b.is_ascii_lowercase()) {
        return None;
    }
    let key_len = key_bytes.len();

    let out = s
        .bytes()
        .enumerate()
        .map(|(i, b)| (b'a' + ((b + key_bytes[i % key_len] - 2 * b'a') % 26)) as char)
        .collect::<String>();

    Some(out)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let key_bytes = key.as_bytes();
    if key_bytes.is_empty() || !key_bytes.iter().all(|b| b.is_ascii_lowercase()) {
        return None;
    }
    let key_len = key_bytes.len();

    let out = s
        .bytes()
        .enumerate()
        .map(|(i, b)| (b'a' + ((b + 26 - key_bytes[i % key_len]) % 26)) as char)
        .collect::<String>();

    Some(out)
}

pub fn encode_random(s: &str) -> (String, String) {
    let len = rand::random_range(100..1000);
    let mut key = String::new();
    key.reserve(len);

    for _ in 0..len {
        let b = b'a' + rand::random_range(0..=25);
        key.push(b as char);
    }

    let out = encode(key.as_str(), s);
    (key, out.unwrap())
}
