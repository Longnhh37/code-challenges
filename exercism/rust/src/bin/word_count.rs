use std::collections::HashMap;
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let bytes = words.as_bytes();
    let (mut map, cur) = bytes.iter().enumerate().fold(
        (HashMap::new(), String::new()),
        |(mut map, mut cur), (i, &b)| {
            let b = b.to_ascii_lowercase();
            match b {
                b'\''
                    if i != 0
                        && bytes[i - 1].is_ascii_alphabetic()
                        && i != bytes.len() - 1
                        && bytes[i + 1].is_ascii_alphabetic() =>
                {
                    cur.push(b as char);
                }
                b'a'..=b'z' | b'0'..=b'9' => cur.push(b as char),
                _ => {
                    if !cur.is_empty() {
                        *map.entry(cur.clone()).or_default() += 1;
                        cur.clear();
                    }
                }
            }

            (map, cur)
        },
    );

    if !cur.is_empty() {
        *map.entry(cur.clone()).or_default() += 1;
    }

    map
}
fn main() {}
