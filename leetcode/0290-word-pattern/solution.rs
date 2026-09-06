use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut bmap: HashMap<u8, String> = HashMap::new();       
        let mut wmap: HashMap<String, u8> = HashMap::new();       

        let pat = pattern.as_bytes();
        let s: Vec<String> = s.split_ascii_whitespace().map(|s| s.to_string()).collect();
        
        if pat.len() != s.len() {
            return false;
        }
        for i in 0..pat.len() {
            let b = pat[i];
            let w = s[i].clone();

            let contains_b = bmap.contains_key(&b);
            let contains_w = wmap.contains_key(&w);

            if contains_b && !contains_w
            || !contains_b && contains_w
            || contains_b && contains_w && (bmap.get(&b).unwrap() != &w || wmap.get(&w).unwrap() != &b) {
                return false;
            } else {
                bmap.insert(b, w.clone());
                wmap.insert(w, b);
            }
        }

    true
    }
}
