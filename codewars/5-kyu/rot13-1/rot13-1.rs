const ALPHABETS_LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const ALPHABETS_UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
​
fn rot13(message: &str) -> String {
    let mut res = String::with_capacity(message.len());
    
    for c in message.chars() {
        if c.is_ascii_lowercase() {
            let c = ALPHABETS_LOWER[((c as u8 - b'a') as usize + 13) % 26] as char;
            res.push(c);
        } else if c.is_ascii_uppercase() {
            let c = ALPHABETS_UPPER[((c as u8 - b'A') as usize + 13) % 26] as char;
            res.push(c);
        } else {
            res.push(c);
        }
    }
    
    res
    
}
​