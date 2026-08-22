mod preloaded;
use preloaded::MORSE_CODE; // MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".
​
fn decode_morse(encoded: &str) -> String {
    let mut res = String::new();
    let mut cur = String::new();
    let mut it = encoded.trim().bytes().peekable();
    
    while let Some(b) = it.next() {
        if b != b' ' {
            cur.push(b as char);
            continue;
        }
        
        let w = MORSE_CODE.get(&cur).unwrap();
        res.push_str(&w);
        cur.clear();
        
        if it.peek() == Some(&b' ') {
            res.push(' ');
            it.next();
            it.next();
        }
    } 
    
    if !cur.is_empty() {
        res.push_str(MORSE_CODE.get(&cur).unwrap());
    }
    
    res
}