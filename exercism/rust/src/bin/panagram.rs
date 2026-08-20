pub fn is_pangram(sentence: &str) -> bool {
    let mut mask = 0u32;

    for b in sentence.bytes() {
        if b.is_ascii_alphabetic() {
            let bit = (b.to_ascii_lowercase() - b'a') as u32;
            mask |= 1 << bit;

            if mask == (1 << 26) - 1 {
                return true;
            }
        }
    }

    false
}
fn main() {}
