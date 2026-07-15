pub fn encode(plain: &str) -> String {
    plain
        .bytes()
        .filter_map (|b| match b {
                b'a'..=b'z' => Some(b'z' - b + b'a'),
                b'A'..=b'Z' => Some(b'z' - (b | 0b0010_0000) + b'a'),
                b'0'..=b'9' => Some(b),
                _ => None,
            })
        .enumerate()
        .flat_map(|(i, b)| {
            let ch = b as char;
            (i > 0 && i.is_multiple_of(5))
            .then(|| ' ')
            .into_iter()
            .chain(std::iter::once(ch))
        })
        .collect::<String>()
}

pub fn decode(cipher: &str) -> String {
    cipher
        .bytes()
        .filter_map(|b| 
            match b {
                b'a'..=b'z' => Some((b'z' - b + b'a') as char),
                b'0'..=b'9' => Some(b as char),
                _ => None,
        })
        .collect::<String>()

}