pub fn translate(input: &str) -> String {
    fn translate_word(word: &str) -> String {
        let bytes = word.as_bytes();

        if matches!(bytes[0], b'a' | b'i' | b'u' | b'e' | b'o')
            || bytes.starts_with(b"xr")
            || bytes.starts_with(b"yt")
        {
            return format!("{word}ay");
        }

        for i in 0..bytes.len() {
            match bytes[i] {
                b'q' if i + 1 < bytes.len() && bytes[i + 1] == b'u' => {
                    return format!("{}{}ay", &word[i + 2..], &word[..i + 2]);
                }

                b'y' if i >= 1 => {
                    return format!("{}{}ay", &word[i..], &word[..i]);
                }

                b'a' | b'i' | b'u' | b'e' | b'o' => {
                    return format!("{}{}ay", &word[i..], &word[..i]);
                }

                _ => {}
            }
        }

        format!("{word}ay")
    }

    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
