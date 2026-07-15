pub fn abbreviate(phrase: &str) -> String {
    let mut out = String::new();

    if phrase.is_empty() {
        return out;
    }

    let bytes = phrase.bytes();

    let mut beg_of_word = true;
    let mut prev_char_is_upper = true;

    for b in bytes {
        if b.is_ascii_lowercase() {
            prev_char_is_upper = false;

            if beg_of_word {
                out.push((b as char).to_ascii_uppercase());
                beg_of_word = false;
            }
        } else if b.is_ascii_uppercase() {
            if beg_of_word || !prev_char_is_upper {
                out.push(b as char);
                beg_of_word = false;
            }
            prev_char_is_upper = true;
        } else if [b' ', b'-'].contains(&b) {
            beg_of_word = true;
            prev_char_is_upper = false;
        } else {
            prev_char_is_upper = false;
        }
    }

    out
}
