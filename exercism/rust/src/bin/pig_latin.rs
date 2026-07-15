pub fn translate(input: &str) -> String {
    fn is_vowel(b: u8) -> bool {
        matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    fn translate_word(word: &str, out: &mut String) {
        let bytes = word.as_bytes();
        let len = bytes.len();

        // case 1: vowel / xr / yt
        if is_vowel(bytes[0]) || bytes.starts_with(b"xr") || bytes.starts_with(b"yt") {
            out.push_str(word);
            out.push_str("ay");
            return;
        }

        // tìm split point
        let mut i = 0;
        while i < len {
            match bytes[i] {
                b'q' if i + 1 < len && bytes[i + 1] == b'u' => {
                    i += 2;
                    break;
                }
                b'y' if i >= 1 => break,
                b if is_vowel(b) => break,
                _ => i += 1,
            }
        }

        // rotate + append
        out.push_str(&word[i..]);
        out.push_str(&word[..i]);
        out.push_str("ay");
    }

    // ⚡ pre-allocate gần đúng
    let mut out = String::with_capacity(input.len() + input.len() / 2);

    for (idx, word) in input.split_ascii_whitespace().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        translate_word(word, &mut out);
    }

    out
}
fn main() {}
