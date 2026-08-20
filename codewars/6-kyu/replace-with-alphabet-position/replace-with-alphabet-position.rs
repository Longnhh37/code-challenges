fn alphabet_position(text: &str) -> String {
    let mut res = String::new();
    for b in text.bytes() {
        if !b.is_ascii_alphabetic() {
            continue;
        }
        let b = b.to_ascii_lowercase();
        res.push_str(&(b - b'a' + 1).to_string());
        res.push(' ');
    }
    res.pop();
    res
}