use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let chars: Vec<u8> = pattern.as_bytes().to_vec();
        let words: Vec<&str> = s.split(' ').collect();

        if chars.len() != words.len() {
            return false;
        }

        let mut c_to_w: HashMap<u8, &str> = HashMap::new();
        let mut w_to_c: HashMap<&str, u8> = HashMap::new();

        for (&c, &w) in chars.iter().zip(words.iter()) {
            match (c_to_w.get(&c), w_to_c.get(w)) {
                (Some(&mapped_w), Some(&mapped_c)) => {
                    if w != mapped_w || c != mapped_c {
                        return false;
                    }
                }
                (None, None) => {
                    c_to_w.insert(c, w);
                    w_to_c.insert(w, c);
                }
                _ => return false,
            }
        }

        true
    }
}
