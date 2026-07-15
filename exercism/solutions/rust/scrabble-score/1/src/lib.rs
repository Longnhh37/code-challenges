pub fn score(word: &str) -> u64 {
    let mut total = 0;

    for b in word.bytes() {
        if !b.is_ascii_alphabetic() {
            continue;
        }
        let b = b.to_ascii_lowercase();
        match b {
            b'a' | b'e' | b'i' | b'u' | b'o' | b'l' | b'n' | b'r' | b's' | b't' => total += 1,
            b'd' | b'g' => total += 2,
            b'b'| b'c'| b'm'| b'p' => total += 3,
            b'f'| b'h'| b'v' | b'w' | b'y' => total += 4,
            b'k' => total += 5,
            b'j' | b'x' => total += 8,
            b'q' | b'z' => total += 10,
            _ => unreachable!(),
        }
    }
        
    total
}
