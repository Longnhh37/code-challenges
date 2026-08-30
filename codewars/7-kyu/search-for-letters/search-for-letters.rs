fn change(string: &str) -> String {
    let mut check = vec![b'0'; 26];
    for b in string.bytes() {
        if b.is_ascii_alphabetic() {
            check[(b.to_ascii_lowercase() - b'a') as usize] = b'1';
       }
    }
    String::from_utf8(check).unwrap()
}