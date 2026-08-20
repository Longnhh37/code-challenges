fn wave(s: &str) -> Vec<String> {
    let mut res = Vec::with_capacity(s.len());
    let bytes = s.as_bytes();
    
    for i in 0..bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            continue;
        }
        
        let mut cur: Vec<u8> = s.bytes().collect();
        cur[i] = cur[i].to_ascii_uppercase();
        res.push(String::from_utf8(cur).unwrap());
    }
    res
}